# WebGL2 pipeline check

`check.sh` builds bevy_carnage's `carnage_web` example for `wasm32-unknown-unknown` with Bevy's
`webgl2` backend (not `webgpu`), serves it, opens it in headless Chrome, optionally fires shots at
the screen, and checks the result. Run it after any change to `src/flesh.wesl` (CLAUDE.md requires
this): it is the only check that compiles the shader through naga's WGSL-to-GLSL path for WebGL2.

```
tools/webgl2/check.sh <source-tree> [settle-seconds]
CLICKS="600,230;470,220" tools/webgl2/check.sh <source-tree> 30
```

- `<source-tree>`: a bevy_carnage checkout. Its `src/`, `examples/`, `tests/`, `benches/`,
  `Cargo.toml` and `Cargo.lock` are copied into a worktree; the source tree is never written.
- `settle-seconds` (default 12): how long the scene runs after init before the screenshot.
- `CLICKS="x,y;x,y"`: a left click at each point after settling. Each click fires a shot, so blood is
  drawn through the flesh shader before the screenshot.

Environment (all optional; the script writes only inside these three):

| Variable | Default | Holds |
|---|---|---|
| `WEBGL2_WORK` | `$HOME/.cache/bevy_carnage_webgl2` | tools (wasm-bindgen), `www/`, `runs/`, logs, screenshots |
| `WEBGL2_WORKTREE` | `$WEBGL2_WORK/worktree` | detached git worktree the wasm build runs in |
| `WEBGL2_TARGET` | `$WEBGL2_WORK/target` | cargo target dir for the wasm build |

When the worktree is missing, the script runs `git -C <source-tree> worktree add --detach <path> HEAD`
and prints that it did. Remove it when finished with `git -C <source-tree> worktree remove <path>`.

## Verdict

- `WEBGL2 PIPELINE OK`, exit 0: the module initialised, the page reports a WebGL2 context, no console
  line matches the error patterns, and the screenshot has at least 64 distinct colours and no single
  colour above 99 % of pixels.
- `WEBGL2 PIPELINE FAIL` with reasons, exit 1: `module-not-initialised`, `no-webgl2-context`,
  `error-lines`, `blank-or-missing-screenshot`, or `wasm-build-failed`.
- `HARNESS ERROR`, exit 2: a missing prerequisite, a bad source tree, or the driver failing.

The screenshot is decoded by a standard-library PNG reader in `check.sh`, so no Pillow is needed.
The console log and the screenshot are kept under `$WEBGL2_WORK`; each run's console log is also
copied to `$WEBGL2_WORK/runs/`.

## What it checks, and what it cannot

It checks that the wasm module compiles for WebGL2, that the shader links and the scene initialises
in a real browser engine, that no error reaches the console, and that something non-blank is drawn.

It cannot check:
- real GPUs or drivers. Chrome runs on SwiftShader, a software WebGL2 implementation.
- the 16-texture limit that matters on real hardware. SwiftShader does not enforce a real device's
  limit.
- whether blood looks right. The screenshot is checked for content, not appearance. Judge the look
  by eye, from the screenshot or a recording.

## Prerequisites

- `rustup` and `cargo`. The script adds the `wasm32-unknown-unknown` target when absent.
- `git`, `rsync`, `perl`, `shasum`, `curl` (all standard on macOS).
- `python3` (standard library only).
- Node 22 or newer. `shoot.mjs` uses the global `WebSocket` and `fetch`; the script checks the version.
- Google Chrome at `/Applications/Google Chrome.app`, or `google-chrome` or `chromium` on `PATH`.
- `wasm-bindgen-cli` is installed by the script into `$WEBGL2_WORK/tools` at the version of the
  `wasm-bindgen` crate in the source tree's `Cargo.lock`, on first use or when that version changes.

## Timings

Measured on this host (the first wasm build of bevy_carnage in a fresh target dir):

- Cold: about 38 minutes (the wasm build was 2317 s).
- Incremental (warm target, changed shader or source): 30 seconds to a few minutes.

## Proof that it can fail

A copy of `src/flesh.wesl` with this line added inside the fragment function fails the build:

```
let deliberately_broken: i32 = d;
```

The build log reports naga's type error:

```
the type of `deliberately_broken` is expected to be `i32`, but got `vec4<f32>`
```

To reproduce, copy the source tree to a scratch directory, make that one edit to the copy's
`src/flesh.wesl`, and run the script on the copy. Do not edit the real tree.
