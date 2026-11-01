# Media: every image, with the command that made it

The clips under `docs/` are listed with their recorders and regeneration commands in [`docs/DEMOS.md`](../DEMOS.md#regenerating-these); they stay at their paths, because the READMEs of published releases name them. This folder holds what was added after that list:

| File | What it shows | Command | Captured | Shows |
|---|---|---|---|---|
| `docs/media/preset.png` | the README's still: frame 120 of `docs/preset.gif`, the body in pieces in its pool | `ffmpeg -i docs/preset.gif -vf "select=eq(n\,120)" -frames:v 1 docs/media/preset.png` | 2026-10-31 | 0.5.x (from `preset.gif`, recorded 2026-10-12) |
| `docs/media/floor_spray.png` | the floor in front of the mannequin after the clip's three shots, three panels side by side: 0.7.0 at 256 texels (five jelly blobs), this change at 256 (a fan of about forty stains) and at 512 (about fifty finer ones) | `capture_skinned --eye 0.0,1.5,3.1 --at 0,0,1.3 [--floor 512]`, frame 359, each cropped to 500 x 200 at (160, 110); labels added with Pillow | 2026-10-31 | the release after 0.7.0 |
| `docs/media/drops.png` | four crops of frame 359, two rows by two: the floor at the shipped 256 texels (top) and the wall of `--through` (bottom), each as 0.8.1 drew it (left) and as `paint_cone` draws it (right) | `capture_skinned --eye 0.0,1.5,3.1 --at 0,0,1.3` and `capture_skinned --through`, on `0.8.1` and on the release after it, each cropped and enlarged 3x (nearest) to the same width; labels added with Pillow | 2026-10-31 | 0.8.2 |
