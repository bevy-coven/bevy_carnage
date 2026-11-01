#!/usr/bin/env bash
# WebGL2 pipeline check for bevy_carnage's `carnage_web` example (src/flesh.wesl on WebGL2).
#
# usage: tools/webgl2/check.sh <source-tree> [settle-seconds]
#
#   <source-tree>     a bevy_carnage checkout. Its src/ examples/ tests/ benches/ Cargo.toml and
#                     Cargo.lock are copied into the worktree; the source tree itself is never written.
#   [settle-seconds]  seconds the scene runs after init before the screenshot (default 12).
#   CLICKS="x,y;x,y"  optional: a left click at each point after settling, so shots are fired and
#                     blood is drawn through the flesh shader before the screenshot.
#
# Environment (all optional; the script writes only inside these three):
#   WEBGL2_WORK      work dir: tools/ (wasm-bindgen), www/, runs/, logs, screenshots
#                    (default $HOME/.cache/bevy_carnage_webgl2)
#   WEBGL2_WORKTREE  detached git worktree the wasm build runs in (default $WEBGL2_WORK/worktree)
#   WEBGL2_TARGET    cargo target dir for the wasm build (default $WEBGL2_WORK/target)
#
# Verdict: `WEBGL2 PIPELINE OK` only when the module initialised, the page had a WebGL2 context,
# no console line matches the error patterns, and the screenshot is non-blank.
# Exit: 0 OK, 1 FAIL, 2 usage or harness error.
set -uo pipefail

START_ALL=$(date +%s)
SELF_DIR=$(cd "$(dirname "$0")" && pwd)
SRC_ARG=${1:-}
SETTLE=${2:-12}

die() { echo "HARNESS ERROR: $*" >&2; exit 2; }
[ -n "$SRC_ARG" ] || { echo "usage: $0 <source-tree> [settle-seconds]" >&2; exit 2; }
SRC=$(cd "$SRC_ARG" 2>/dev/null && pwd) || { echo "no such source tree: $SRC_ARG" >&2; exit 2; }
[ -f "$SRC/Cargo.toml" ] && [ -f "$SRC/Cargo.lock" ] || die "$SRC is not a bevy_carnage tree (Cargo.toml or Cargo.lock missing)"

WORK=${WEBGL2_WORK:-$HOME/.cache/bevy_carnage_webgl2}
WT=${WEBGL2_WORKTREE:-$WORK/worktree}
TARGET=${WEBGL2_TARGET:-$WORK/target}
TOOLS=$WORK/tools
WB=$TOOLS/bin/wasm-bindgen
WWW=$WORK/www
RUNS=$WORK/runs
LOG=$WORK/console.log
SHOT=$WORK/shot-$(date +%Y%m%d-%H%M%S).png
SERVER_PID=""
CHROME_PROFILE=""

cleanup() {
  [ -n "$SERVER_PID" ] && kill "$SERVER_PID" 2>/dev/null
  if [ -n "$CHROME_PROFILE" ]; then
    pkill -f -- "--user-data-dir=$CHROME_PROFILE" 2>/dev/null
    sleep 1
    rm -rf "$CHROME_PROFILE" 2>/dev/null
  fi
  true
}
trap cleanup EXIT

# --- 0. prerequisites ---------------------------------------------------------------------------
need() { command -v "$1" >/dev/null 2>&1 || die "missing prerequisite '$1': $2"; }
need git "source checkout and worktree"
need rsync "file sync into the worktree"
need perl "Cargo.toml feature swap"
need shasum "source fingerprint"
need curl "server readiness probe"
need rustup "installs the wasm32 target"
need cargo "builds the wasm module"
need python3 "serves the page and checks the screenshot (standard library only)"
need node "drives Chrome over CDP (Node 22+)"
[ "$(node -e 'console.log(Number(process.versions.node.split(".")[0]) >= 22 ? "yes" : "no")')" = yes ] \
  || die "node $(node --version) is too old: shoot.mjs needs Node 22+ (global WebSocket and fetch)"

find_chrome() {
  if [ -x "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" ]; then
    echo "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"; return
  fi
  for c in google-chrome chromium; do
    if command -v "$c" >/dev/null 2>&1; then command -v "$c"; return; fi
  done
}
CHROME=$(find_chrome)
[ -n "$CHROME" ] || die "no Chrome: install Google Chrome, or put google-chrome or chromium on PATH"

mkdir -p "$WORK" "$TOOLS" "$WWW" "$RUNS" || die "cannot create work dir $WORK"

# The worktree: created detached at the source tree's HEAD when missing. git records it in the
# source repo's .git (git's own metadata); no source file is touched.
if [ ! -e "$WT/.git" ]; then
  if [ -e "$WT" ] && [ -n "$(ls -A "$WT" 2>/dev/null)" ]; then
    die "$WT exists, is not a git worktree, and is not empty"
  fi
  echo "worktree: creating $WT (detached at $(git -C "$SRC" rev-parse --short HEAD))"
  git -C "$SRC" worktree add --detach "$WT" HEAD || die "git worktree add failed"
  echo "worktree: cleanup when finished, run inside the source tree: git worktree remove $WT"
fi

# wasm-bindgen must match the wasm-bindgen crate in the source tree's Cargo.lock exactly.
WB_VER=$(grep -A1 '^name = "wasm-bindgen"$' "$SRC/Cargo.lock" | sed -n 's/^version = "\(.*\)"$/\1/p' | sed -n 1p)
[ -n "$WB_VER" ] || die "no wasm-bindgen entry in $SRC/Cargo.lock"
HAVE_WB=$("$WB" --version 2>/dev/null | cut -d' ' -f2)
if [ "$HAVE_WB" != "$WB_VER" ]; then
  echo "install: wasm-bindgen-cli $WB_VER into $TOOLS (several minutes on first install)"
  cargo install --root "$TOOLS" --locked wasm-bindgen-cli --version "$WB_VER" || die "cargo install wasm-bindgen-cli failed"
fi
[ "$("$WB" --version | cut -d' ' -f2)" = "$WB_VER" ] || die "$WB is not wasm-bindgen $WB_VER"

if ! rustup target list --installed | grep -qx wasm32-unknown-unknown; then
  echo "rustup: adding wasm32-unknown-unknown"
  rustup target add wasm32-unknown-unknown || die "rustup target add failed"
fi

# --- 1. sync the source tree into the worktree --------------------------------------------------
# Cargo decides what to rebuild from mtimes, and a synced file's mtime cannot be trusted: an
# older copy of flesh.wesl with a stale mtime was once reused by cargo (a 2 s build and an OK
# verdict on a broken shader). So: copy changed content (rsync -c gives a changed file a fresh
# mtime), then fingerprint the source tree and, when it differs from the last sync, touch every
# synced file so cargo rebuilds the crate.
put_if_changed() { cmp -s "$1" "$2" || { cp "$1" "$2.tmp" && mv "$2.tmp" "$2"; }; }
for d in src examples tests benches; do
  if [ -d "$SRC/$d" ]; then
    mkdir -p "$WT/$d"
    rsync -rc --delete --no-times "$SRC/$d/" "$WT/$d/" || die "rsync $d failed"
  fi
done
put_if_changed "$SRC/Cargo.lock" "$WT/Cargo.lock" || die "copying Cargo.lock failed"

# --- 2. webgl2 swap (worktree Cargo.toml only) --------------------------------------------------
# Bevy's wasm dev-dependency selects `webgpu`, which wins over `webgl2` in bevy_render's
# WgpuSettings::default (settings.rs). Swap it so the GL backend is the only one on wasm.
SWAPPED=$WORK/Cargo.toml.swapped
perl -0pe 's/(\[target\.\x27cfg\(target_arch = "wasm32"\)\x27\.dev-dependencies\]\nbevy = \{ version = "0\.20", default-features = false, features = \[)"web", "webgpu"\]/$1"web", "webgl2"]/' "$SRC/Cargo.toml" > "$SWAPPED" || die "perl swap failed"
put_if_changed "$SWAPPED" "$WT/Cargo.toml" || die "copying Cargo.toml failed"
grep -q 'features = \["web", "webgl2"\] }' "$WT/Cargo.toml" || die "webgl2 swap did not apply (the wasm dev-dependency line in $SRC/Cargo.toml changed?)"
grep -q '"webgpu"\] }' "$WT/Cargo.toml" && die "webgpu still selected in $WT/Cargo.toml"

# Fingerprint of the SOURCE tree (not the worktree, which also holds the swapped Cargo.toml).
# `find` exits 1 when a directory such as benches/ is absent, so its status is ignored.
fingerprint() {
  { find src examples tests benches Cargo.toml Cargo.lock -type f 2>/dev/null || true; } | LC_ALL=C sort | xargs shasum -a 256 | shasum -a 256 | cut -d' ' -f1
}
MANIFEST=$(cd "$SRC" && fingerprint)
if [ "$MANIFEST" != "$(cat "$WORK/sync.manifest" 2>/dev/null)" ]; then
  (cd "$WT" && { find src examples tests benches Cargo.toml Cargo.lock -type f 2>/dev/null || true; } | xargs touch) || die "touch failed"
  printf '%s\n' "$MANIFEST" > "$WORK/sync.manifest"
  echo "sync: source changed since last sync; touched synced files so cargo rebuilds"
fi

# --- 3. build -----------------------------------------------------------------------------------
cd "$WT" || die "cd $WT"
T0=$(date +%s)
CARGO_TARGET_DIR="$TARGET" cargo build --release --example carnage_web --target wasm32-unknown-unknown >"$WORK/build.log" 2>&1
BUILD_RC=$?
T1=$(date +%s)
echo "build: exit $BUILD_RC in $((T1 - T0)) s (log $WORK/build.log)"
if [ $BUILD_RC -ne 0 ]; then
  grep -E '^error' -A6 "$WORK/build.log" | head -80
  echo "WEBGL2 PIPELINE FAIL"
  echo "reasons: wasm-build-failed;"
  exit 1
fi
WASM=$TARGET/wasm32-unknown-unknown/release/examples/carnage_web.wasm
[ -f "$WASM" ] || die "no wasm at $WASM"

# --- 4. wasm-bindgen + page ---------------------------------------------------------------------
"$WB" --target web --out-dir "$WWW" --out-name carnage_web "$WASM" || die "wasm-bindgen failed"
cat > "$WWW/index.html" <<'HTML'
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>carnage_web WebGL2 check</title>
<link rel="icon" href="data:,">
<style>
html, body { margin: 0; height: 100%; background: #000; overflow: hidden; }
canvas#carnage-canvas { display: block; width: 100vw; height: 100vh; }
</style>
</head>
<body>
<canvas id="carnage-canvas"></canvas>
<script>
// Everything the page says goes to the server, one POST per message, prefixed with its level.
(function () {
  function post(level, text) {
    try { fetch('/log', { method: 'POST', body: '[' + level + '] ' + text, keepalive: true }); } catch (e) {}
  }
  function show(a) {
    if (a instanceof Error) return a.stack || String(a);
    if (a && typeof a === 'object') { try { return JSON.stringify(a); } catch (e) { return String(a); } }
    return String(a);
  }
  ['log', 'info', 'warn', 'error', 'debug'].forEach(function (k) {
    var orig = console[k].bind(console);
    console[k] = function () {
      post('console.' + k, Array.prototype.map.call(arguments, show).join(' '));
      orig.apply(null, arguments);
    };
  });
  window.addEventListener('error', function (ev) {
    post('onerror', (ev.message || '') + ' at ' + ev.filename + ':' + ev.lineno + ' ' + (ev.error ? show(ev.error) : ''));
  });
  window.addEventListener('unhandledrejection', function (ev) {
    post('unhandledrejection', show(ev.reason));
  });
  var probe = document.createElement('canvas');
  var gl2 = probe.getContext('webgl2');
  window.__webgl2 = !!gl2;
  if (gl2) {
    var dbg = gl2.getExtension('WEBGL_debug_renderer_info');
    post('page', 'webgl2 context: yes; renderer=' + (dbg ? gl2.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : '?') +
      '; version=' + gl2.getParameter(gl2.VERSION));
  } else {
    post('page', 'webgl2 context: no');
  }
  post('page', 'page script ready; importing carnage_web.js');
})();
</script>
<script type="module">
  import init from './carnage_web.js';
  init().then(function () {
    console.log('init resolved');
  }, function (e) {
    console.error('init rejected: ' + (e && (e.stack || e)));
  });
</script>
</body>
</html>
HTML

# --- 5. server + Chrome -------------------------------------------------------------------------
PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1])')
CDP=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1])')
: > "$LOG"
python3 "$SELF_DIR/serve.py" "$WWW" "$PORT" "$LOG" >"$WORK/serve.stderr" 2>&1 &
SERVER_PID=$!
for _ in $(seq 1 50); do
  curl -fsS -o /dev/null "http://127.0.0.1:$PORT/index.html" 2>/dev/null && break
  sleep 0.2
done
curl -fsS -o /dev/null "http://127.0.0.1:$PORT/index.html" || die "server did not come up on $PORT"

CHROME_PROFILE=$WORK/profile-$$
mkdir -p "$CHROME_PROFILE"
"$CHROME" --headless=new --no-sandbox --use-angle=swiftshader --enable-unsafe-swiftshader \
  --ignore-gpu-blocklist --enable-webgl --window-size=1280,800 --no-first-run \
  --no-default-browser-check --disable-background-networking --remote-debugging-port="$CDP" \
  --user-data-dir="$CHROME_PROFILE" about:blank >"$WORK/chrome.stderr" 2>&1 &

# --- 6. drive the page --------------------------------------------------------------------------
DRIVE_JSON=$(node "$SELF_DIR/shoot.mjs" "$CDP" "http://127.0.0.1:$PORT/index.html" "$LOG" "$SETTLE" "$SHOT" 240)
DRIVE_RC=$?
echo "drive: $DRIVE_JSON (rc $DRIVE_RC)"
[ $DRIVE_RC -eq 0 ] || die "shoot.mjs failed (rc $DRIVE_RC): $DRIVE_JSON"
LOADED=$(node -e 'try { const j = JSON.parse(process.argv[1]); console.log(j.loaded === true ? "yes" : "no"); } catch (e) { console.log("no"); }' "$DRIVE_JSON")
WEBGL2=$(node -e 'try { const j = JSON.parse(process.argv[1]); console.log(j.page && j.page.webgl2 === true ? "yes" : "no"); } catch (e) { console.log("no"); }' "$DRIVE_JSON")

# --- 7. screenshot check (python3 standard library: PNG decoded by hand) ------------------------
read -r -d '' SHOT_PY <<'PY'
import struct, sys, zlib
from collections import Counter

def read_png(path):
    data = open(path, "rb").read()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("not a PNG")
    pos, idat, ihdr = 8, [], None
    while pos + 8 <= len(data):
        length, = struct.unpack(">I", data[pos:pos + 4])
        kind = data[pos + 4:pos + 8]
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b"IHDR":
            ihdr = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat.append(body)
        elif kind == b"IEND":
            break
    if ihdr is None:
        raise ValueError("no IHDR")
    w, h, depth, ctype, _, _, interlace = ihdr
    channels = {0: 1, 2: 3, 4: 2, 6: 4}.get(ctype)
    if depth != 8 or channels is None or interlace != 0:
        raise ValueError("unsupported PNG: depth %d, colour type %d, interlace %d" % (depth, ctype, interlace))
    stride = w * channels
    raw = zlib.decompress(b"".join(idat))
    out = bytearray(h * stride)
    prev = bytearray(stride)
    p = 0
    for y in range(h):
        f = raw[p]
        line = bytearray(raw[p + 1:p + 1 + stride])
        p += 1 + stride
        if f == 1:
            for i in range(channels, stride):
                line[i] = (line[i] + line[i - channels]) & 255
        elif f == 2:
            for i in range(stride):
                line[i] = (line[i] + prev[i]) & 255
        elif f == 3:
            for i in range(stride):
                left = line[i - channels] if i >= channels else 0
                line[i] = (line[i] + ((left + prev[i]) >> 1)) & 255
        elif f == 4:
            for i in range(stride):
                a = line[i - channels] if i >= channels else 0
                b = prev[i]
                c = prev[i - channels] if i >= channels else 0
                pa, pb, pc = abs(b - c), abs(a - c), abs(a + b - 2 * c)
                pred = a if pa <= pb and pa <= pc else (b if pb <= pc else c)
                line[i] = (line[i] + pred) & 255
        elif f != 0:
            raise ValueError("bad PNG filter %d" % f)
        out[y * stride:(y + 1) * stride] = line
        prev = line
    return w, h, channels, out

try:
    w, h, ch, px = read_png(sys.argv[1])
    if ch == 1:
        colours = Counter(px)
    elif ch == 2:
        g = px[0::2]
        colours = Counter(zip(g, g, g))
    else:
        colours = Counter(zip(px[0::ch], px[1::ch], px[2::ch]))
    total = w * h
    distinct = len(colours)
    top = max(colours.values()) / total
    nonblank = distinct >= 64 and top < 0.99
    print("SIZE %dx%d" % (w, h))
    print("DISTINCT %d" % distinct)
    print("TOP_COLOUR_SHARE %.4f" % top)
    print("NONBLANK %s" % ("yes" if nonblank else "no"))
except Exception as e:
    print("PNG-ERROR %s" % e)
    print("NONBLANK no")
PY

# --- 8. verdict ---------------------------------------------------------------------------------
ERRS=$(grep -Eiv 'favicon' "$LOG" | grep -Ei '(^|[^a-z])(error|panic|validation|failed|too many|unsupported|invalid)([^a-z]|$)' || true)
DISTINCT=0; NONBLANK=no; PNG_ERR=""
if [ -f "$SHOT" ]; then
  SHOTOUT=$(python3 -c "$SHOT_PY" "$SHOT" 2>&1)
  NONBLANK=$(printf '%s\n' "$SHOTOUT" | sed -n 's/^NONBLANK //p' | sed -n 1p)
  DISTINCT=$(printf '%s\n' "$SHOTOUT" | sed -n 's/^DISTINCT //p' | sed -n 1p)
  PNG_ERR=$(printf '%s\n' "$SHOTOUT" | sed -n 's/^PNG-ERROR //p')
fi

REASONS=""
[ "$LOADED" = yes ] || REASONS="$REASONS module-not-initialised;"
[ "$WEBGL2" = yes ] || REASONS="$REASONS no-webgl2-context;"
[ -z "$ERRS" ] || REASONS="$REASONS error-lines;"
[ "$NONBLANK" = yes ] || REASONS="$REASONS blank-or-missing-screenshot;"

echo "console lines: $(wc -l < "$LOG" | tr -d ' ') (log $LOG)"
echo "page webgl2: $WEBGL2   module initialised: $LOADED"
echo "screenshot: ${SHOT}  distinct colours: ${DISTINCT:-0}  nonblank: ${NONBLANK:-no}"
[ -z "$PNG_ERR" ] || echo "screenshot decode: $PNG_ERR"
if [ -n "$ERRS" ]; then
  echo "matching console lines:"
  printf '%s\n' "$ERRS" | head -60
fi
mkdir -p "$RUNS" && cp "$LOG" "$RUNS/$(basename "$SHOT" .png).console.log"
if [ -z "$REASONS" ]; then
  echo "WEBGL2 PIPELINE OK"
  VERDICT=OK
else
  echo "WEBGL2 PIPELINE FAIL"
  echo "reasons: $REASONS"
  VERDICT=FAIL
fi
echo "total: $(( $(date +%s) - START_ALL )) s"
[ "$VERDICT" = OK ] && exit 0 || exit 1
