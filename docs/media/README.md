# Media: every image, with the command that made it

The clips under `docs/` are listed with their recorders and regeneration commands in [`docs/DEMOS.md`](../DEMOS.md#regenerating-these); they stay at their paths, because the READMEs of published releases name them. This folder holds what was added after that list:

| File | What it shows | Command | Captured | Shows |
|---|---|---|---|---|
| `docs/media/preset.png` | the README's still: frame 120 of `docs/preset.gif`, the body in pieces in its pool | `ffmpeg -i docs/preset.gif -vf "select=eq(n\,120)" -frames:v 1 docs/media/preset.png` | 2026-10-31 | 0.5.x (from `preset.gif`, recorded 2026-10-12) |
