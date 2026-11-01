// CDP driver for the WebGL2 check. Started by check.sh against a Chrome it launched itself.
// Needs Node 22+ (the global WebSocket and fetch); check.sh checks the version before it runs this.
//
// usage: node shoot.mjs <cdp-port> <page-url> <console-log> <settle-seconds> <screenshot.png> <load-timeout-seconds>
//
// Opens the page as a new target, waits until the console log holds "init resolved" (or the load
// timeout passes), lets the scene run for <settle-seconds>, then writes a PNG screenshot. Prints one
// JSON line on stdout. Exit 0 whenever the driver itself ran; the verdict is check.sh's job.
import { existsSync, readFileSync, writeFileSync } from "node:fs";

const [, , cdpPort, url, logPath, settleArg, shotPath, loadTimeoutArg] = process.argv;
const settle = Number(settleArg) * 1000;
const loadTimeout = Number(loadTimeoutArg) * 1000;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

// Hard stop so a wedged Chrome cannot hang the harness.
setTimeout(() => {
  console.log(JSON.stringify({ driver_error: "hard deadline exceeded" }));
  process.exit(3);
}, loadTimeout + settle + 60_000).unref();

let target;
for (let i = 0; i < 150; i++) {
  try {
    target = await fetch(`http://127.0.0.1:${cdpPort}/json/new?${encodeURIComponent(url)}`, {
      method: "PUT",
    }).then((r) => r.json());
    break;
  } catch {
    await sleep(200);
  }
}
if (!target?.webSocketDebuggerUrl) {
  console.log(JSON.stringify({ driver_error: "no CDP target" }));
  process.exit(2);
}

const ws = new WebSocket(target.webSocketDebuggerUrl);
let nextId = 0;
const pending = new Map();
ws.onmessage = (ev) => {
  const m = JSON.parse(ev.data);
  if (m.id && pending.has(m.id)) {
    pending.get(m.id)(m);
    pending.delete(m.id);
  }
};
await new Promise((resolve, reject) => {
  ws.onopen = resolve;
  ws.onerror = reject;
});
const send = (method, params = {}) =>
  new Promise((resolve) => {
    const id = ++nextId;
    pending.set(id, resolve);
    ws.send(JSON.stringify({ id, method, params }));
  });

await send("Page.enable");
await send("Runtime.enable");

const started = Date.now();
let loaded = false;
while (Date.now() - started < loadTimeout) {
  if (existsSync(logPath) && readFileSync(logPath, "utf8").includes("init resolved")) {
    loaded = true;
    break;
  }
  await sleep(500);
}
const loadSeconds = (Date.now() - started) / 1000;
await sleep(Math.min(settle, 4000));
// CLICKS="x,y;x,y": fire a shot at each point (carnage_web: click shoots) so blood is drawn through the flesh shader.
for (const pt of (process.env.CLICKS ?? "").split(";").filter(Boolean)) {
  const [x, y] = pt.split(",").map(Number);
  await send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
  await send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", buttons: 1, clickCount: 1 });
  await send("Input.dispatchMouseEvent", { type: "mouseReleased", x, y, button: "left", buttons: 0, clickCount: 1 });
  await sleep(2000);
}
await sleep(settle);

const probe = await send("Runtime.evaluate", {
  expression:
    'JSON.stringify({ webgl2: window.__webgl2 === true, href: location.href, ' +
    'canvas: [document.querySelector("canvas")?.width ?? null, document.querySelector("canvas")?.height ?? null] })',
  returnByValue: true,
});
const shot = await send("Page.captureScreenshot", { format: "png" });
if (!shot.result?.data) {
  console.log(JSON.stringify({ driver_error: "captureScreenshot returned no data", detail: shot.error ?? null }));
  process.exit(4);
}
writeFileSync(shotPath, Buffer.from(shot.result.data, "base64"));

let page = null;
try {
  page = JSON.parse(probe.result?.result?.value ?? "null");
} catch {
  page = null;
}
console.log(
  JSON.stringify({ loaded, load_seconds: Number(loadSeconds.toFixed(1)), page, shot: shotPath }),
);
ws.close();
process.exit(0);
