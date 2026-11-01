//! THROWAWAY probe (iteration 10): what a wound's pulses leave on a skin canvas, with the real WetCanvas.
//! Config from env `PROBE` = "name:minor:major:spacing:pulses:head_w:head_l;..." (texels), dumps raw amounts to /tmp/c10/probe/<name>.u8
use bevy::prelude::*;
use bevy_carnage::wetmap::{StainShape, WetCanvas, WetSettings};

fn shape(major: f32, minor: f32, size: f32, seed: u32) -> StainShape {
    StainShape { major: major / size, minor: minor / size, spines: 0, satellites: 0, direction: [0.0, 1.0], seed }
}

#[test]
fn probe() {
    let Ok(spec) = std::env::var("PROBE") else { return };
    std::fs::create_dir_all("/tmp/c10/probe").unwrap();
    for cfg in spec.split(';').filter(|s| !s.is_empty()) {
        let f: Vec<&str> = cfg.split(':').collect();
        let name = f[0];
        let p: Vec<f32> = f[1..].iter().map(|v| v.parse().unwrap()).collect();
        let (minor, major, spacing, pulses, head_w, head_l) = (p[0], p[1], p[2], p[3] as u32, p[4], p[5]);
        let size = 256u32;
        let mut images = Assets::<Image>::default();
        let mut c = WetCanvas::new(&mut images, size, [0.8, 0.7, 0.65], 0.6);
        let s = WetSettings::default();
        let sz = size as f32;
        let g = Vec2::new(0.0, 1.0);
        let wells = [-1.0f32, 0.0, 1.0];
        let mut tick = 0u32;
        for pulse in 0..pulses {
            for (k, w) in wells.iter().enumerate() {
                let cx = 128.0 + w * spacing;
                let cy = 40.0 + 0.5 * major;
                c.paint_uv_with(Vec2::new(cx / sz, cy / sz), &shape(major, minor, sz, 7 + k as u32), tick, &s);
                if head_w > 0.0 {
                    c.paint_uv_with(Vec2::new(cx / sz, (40.0 + major - 0.5 * head_l) / sz), &shape(head_l, head_w, sz, 9 + k as u32), tick, &s);
                }
            }
            for _ in 0..50 {
                tick += 1;
                c.tick(tick, g, &s);
            }
            let _ = pulse;
        }
        let mut raw = Vec::with_capacity((size * size) as usize);
        for y in 0..size {
            for x in 0..size {
                raw.push(c.amount_at(x, y));
            }
        }
        std::fs::write(format!("/tmp/c10/probe/{name}.u8"), raw).unwrap();
        println!("probe {name}: digest {:016x}", c.digest());
    }
}
