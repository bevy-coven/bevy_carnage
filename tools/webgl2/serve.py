#!/usr/bin/env python3
"""Static server for the WebGL2 check (tools/webgl2/check.sh).

usage: serve.py <www-dir> <port> <console-log>

Serves <www-dir> (application/wasm for .wasm, no caching). A POST to /log appends its body to
<console-log>, one body per message (the page prefixes each with its console level). Access lines go
to access.log beside the console log, never to the console log itself.
"""
import functools
import http.server
import os
import sys
import threading

root, port, log = sys.argv[1], int(sys.argv[2]), sys.argv[3]
access = os.path.join(os.path.dirname(os.path.abspath(log)), "access.log")
lock = threading.Lock()


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
        ".js": "text/javascript",
        ".mjs": "text/javascript",
    }

    def do_POST(self):
        if self.path != "/log":
            self.send_error(404)
            return
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8", "replace")
        with lock:
            with open(log, "a", encoding="utf-8") as f:
                f.write(body.rstrip("\n") + "\n")
        self.send_response(204)
        self.end_headers()

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def log_message(self, fmt, *args):
        with lock:
            with open(access, "a", encoding="utf-8") as f:
                f.write("%s %s\n" % (self.address_string(), fmt % args))


if __name__ == "__main__":
    server = http.server.ThreadingHTTPServer(
        ("127.0.0.1", port), functools.partial(Handler, directory=root)
    )
    server.serve_forever()
