use bevy::asset::Assets;
use bevy::image::Image;
use bevy::math::Vec2;
use bevy_carnage::wetmap::{WetCanvas, WetSettings};

#[test]
fn probe() {
    let s = WetSettings { dry_ticks: 120, ..Default::default() };
    let mut images = Assets::<Image>::default();
    let mut c = WetCanvas::new(&mut images, 64, [0.78, 0.66, 0.60], 0.55);
    c.paint_cone(Vec2::new(30.5 / 64.0, 30.5 / 64.0), [2.0, 2.0], Vec2::X, 0);
    for t in 1..=400u32 {
        c.tick(t, Vec2::new(0.0, 1.0), &s);
        if [1u32, 40, 80, 119, 120, 121, 200, 400].contains(&t) {
            c.flush(&mut images);
            let alb = images.get(&c.albedo()).unwrap();
            let d = alb.data.as_ref().unwrap();
            let rgh = images.get(&c.roughness()).unwrap();
            let rd = rgh.data.as_ref().unwrap();
            let mut line = format!("t={t:3}");
            for x in 28..=33u32 {
                let i = ((30 * 64 + x) * 4) as usize;
                line += &format!(" | x{x} amt {:3} age {:3} alb ({:3},{:3},{:3}) R {:3} A {:3}", c.amount_at(x, 30), c.age_at(x, 30), d[i], d[i + 1], d[i + 2], rd[i], rd[i + 3]);
            }
            println!("{line}");
        }
    }
}
