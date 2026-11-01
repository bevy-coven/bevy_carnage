// **Flesh: a StandardMaterial extension.** See `src/flesh.rs` for the model and the sources.
//
// Forward path only. In the prepass this writes the ordinary gbuffer; the wrap term is a
// post-lighting correction and a deferred lighting pass would not see it.

import bevy_pbr::render::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_bindings,
    pbr_functions::alpha_discard,
    pbr_types,
    mesh_view_bindings as view_bindings,
    mesh_view_types,
    mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT,
    shadows,
};

@if(PREPASS_PIPELINE)
import bevy_pbr::{
    prepass::io::{VertexOutput, FragmentOutput},
    deferred::functions::deferred_output,
};
@else
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
};

struct FleshParams {
    bands: vec4<f32>,
    wet: vec4<f32>,
    sss: vec4<f32>,
    cut: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> flesh: FleshParams;
// All the tables in one image (`flesh::luts`): the subsurface table in its top-left corner, the blood
// table in the four rows after it and the same blood aged to two stops of its colour walk in the last eight.
// One image because every texture here counts against the sixteen a stage may sample on WebGL2, and a
// camera's environment map takes two more.
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var flesh_luts: texture_2d<f32>;
// One sampler for all four: linear, clamped, level zero — see `FleshExtension`.
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var flesh_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(104) var wet_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(105) var flay_map: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(106) var dermis_map: texture_2d<f32>;

const MODE_CAP: f32 = 0.0;
const MODE_CANVAS: f32 = 1.0;
const MODE_CLOTH: f32 = 2.0;
const MODE_FLOOR: f32 = 3.0;
const FLAG_WET: u32 = 1u;
const FLAG_FLAY: u32 = 2u;
const FLAG_DERMIS: u32 = 4u;
const SSS_ROWS: f32 = 16.0;
const SSS_COLS: f32 = 64.0;
// The packed tables' size, and the rows the blood tables start at: `flesh::LUTS_WIDTH`, `LUTS_HEIGHT`.
// `BLOOD_AGED_ROW0` is the aged table: four rows at the methaemoglobin stop, then four at the end (hemichrome).
const LUTS_W: f32 = 256.0;
const LUTS_H: f32 = 92.0;
const BLOOD_ROW0: f32 = 80.0;
const BLOOD_AGED_ROW0: f32 = 84.0;
// How far through its drying a film is at the first aged stop: `flesh::AGE_KNOT`.
const AGE_KNOT: f32 = 0.35;
const LAYERS: f32 = 5.0;
const SSS_MIN_RADIUS_MM: f32 = 0.5;
const SSS_MAX_RADIUS_MM: f32 = 64.0;
const BLOOD_LUT_MAX_MM: f32 = 1.0;
const PI: f32 = 3.141592653589793;

// **The film depth at which a wet map's rim is drawn**, as a share of the canvas's full depth: 4 %,
// 0.08 mm at the shipped 2 mm. Below it the smoothed field is the blur's own skirt, not blood. This
// crate's own number; at 8 % the thin smear a blade leaves along a cut's lips was cut away.
const WET_EDGE: f32 = 0.04;

// **How much higher a film's surface is drawn than its depth**: 6. A pool's edge is a meniscus a
// millimetre or two wide, far narrower than a canvas texel, so the smoothed depth climbs over a
// texel what the blood climbs over its meniscus; drawn at its true depth, a 2 mm pool on a floor of
// 12 mm texels tilts its edge 10° and catches no light. This crate's own number.
const FILM_RELIEF: f32 = 6.0;

// **How much of `FILM_RELIEF` a film a few texels across is drawn with**: 0.15, where the canvas
// `FILM_SPAN_TEXELS` around a fragment holds under `FILM_SPAN_LOW` of the blood the fragment does,
// rising to all of it at `FILM_SPAN_HIGH`. The relief stands for a meniscus narrower than a texel at the
// edge of a film wider than many; a drop a few texels across has no interior for that edge to bound,
// so lifted it was all edge — a dome — and a wall's spatter read as red beads stuck on. A drop of blood
// spread on a wall is a tenth of a millimetre deep and centimetres across. Measured against the
// fragment's own blood, not as an amount, so a wide film keeps its relief however thin it is. The four
// taps are the canvas itself, bilinear, at the corners of a square `2 · FILM_SPAN_TEXELS` on a side.
// This crate's own numbers.
const FILM_SMALL_RELIEF: f32 = 0.15;
const FILM_SPAN_TEXELS: f32 = 4.0;
const FILM_SPAN_LOW: f32 = 0.25;
const FILM_SPAN_HIGH: f32 = 0.6;

// **The steepest a film's surface is drawn**, as the cosine of its tilt from what it lies on: 25°.
// Without a cap the relief that lifts a floor's 12 mm texels stood a skin's 3.6 mm ones on end; at 35°
// the gaps between a wound's runs, and between the clots in a crater seen close, shaded as dark
// crevices. This crate's own number.
const FILM_MAX_TILT_COS: f32 = 0.906;

// **The share of `FleshParams::cut`'s film a crater's graze wears**: 0.04, reached over the first
// `GRAZE_RAMP` (6 %) of the layer span below the skin, about the skin's own thickness. A fresh
// abrasion is raw and moist — the papillary dermis bleeds and weeps from the first moment — so skin a
// crater only grazed is reddened by a thin film of its own ooze and shines a little; the ramp lets the
// shallowest scuff at the graze's edge fade into the skin rather than stop at the edge of the canvas's
// peeled mask. Bare, as until 0.5.53, the graze was the dermis's pale pink mottled with cream where the
// fat came through, and round every crater it read as a pale halo; at 0.08 with no ramp it was a hard
// red disc. The fact is forensic; the numbers are this crate's own.
const GRAZE_FILM: f32 = 0.04;
const GRAZE_RAMP: f32 = 0.06;

// The cubic B-spline weights of the four texels around a sample at fraction `v` between the middle two.
fn bspline(v: f32) -> vec4<f32> {
    let n = vec4<f32>(1.0, 2.0, 3.0, 4.0) - v;
    let s = n * n * n;
    let x = s.x;
    let y = s.y - 4.0 * s.x;
    let z = s.z - 4.0 * s.y + 6.0 * s.x;
    return vec4<f32>(x, y, z, 6.0 - x - y - z) * (1.0 / 6.0);
}

// **The wet map, reconstructed as a cubic B-spline** in four bilinear taps (Sigg & Hadwiger, "Fast
// third-order texture filtering", GPU Gems 2, ch. 20). A bilinear read of a canvas whose texels are
// wet or not draws every rim as the texels' staircase; the spline is smooth to its second derivative,
// so a level set of it is a curve.
fn wet_smooth(uv: vec2<f32>) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(wet_map));
    let p = uv * size - 0.5;
    let f = fract(p);
    let i = p - f;
    let xc = bspline(f.x);
    let yc = bspline(f.y);
    let c = vec4<f32>(i.x - 0.5, i.x + 1.5, i.y - 0.5, i.y + 1.5);
    let s = vec4<f32>(xc.x + xc.y, xc.z + xc.w, yc.x + yc.y, yc.z + yc.w);
    let o = (c + vec4<f32>(xc.y, xc.w, yc.y, yc.w) / s) / vec4<f32>(size.x, size.x, size.y, size.y);
    let s0 = textureSample(wet_map, flesh_sampler, vec2<f32>(o.x, o.z));
    let s1 = textureSample(wet_map, flesh_sampler, vec2<f32>(o.y, o.z));
    let s2 = textureSample(wet_map, flesh_sampler, vec2<f32>(o.x, o.w));
    let s3 = textureSample(wet_map, flesh_sampler, vec2<f32>(o.y, o.w));
    let sx = s.x / (s.x + s.y);
    let sy = s.z / (s.z + s.w);
    return mix(mix(s3, s2, sx), mix(s1, s0, sx), sy);
}

// Which of the five tissue blocks a depth fraction falls in: 0 skin … 4 marrow.
fn layer_of(depth_frac: f32) -> f32 {
    var layer = 0.0;
    if depth_frac >= flesh.bands.x { layer = 1.0; }
    if depth_frac >= flesh.bands.y { layer = 2.0; }
    if depth_frac >= flesh.bands.z { layer = 3.0; }
    if depth_frac >= flesh.bands.w { layer = 4.0; }
    return layer;
}

// The table row for a sphere radius in mm, log-spaced like `row_radius_mm` in `flesh.rs`.
fn curvature_row(radius_mm: f32) -> f32 {
    if radius_mm >= SSS_MAX_RADIUS_MM {
        return 0.0;
    }
    let lo = log(SSS_MAX_RADIUS_MM);
    let hi = log(SSS_MIN_RADIUS_MM);
    let t = clamp((log(max(radius_mm, SSS_MIN_RADIUS_MM)) - lo) / (hi - lo), 0.0, 1.0);
    return t * (SSS_ROWS - 1.0);
}

// Penner's lookup: the diffuse response of `layer` at `ndotl` on a sphere of `radius_mm`.
// The table is the packed image's top-left corner, each lookup held to its texel centres, as clamping
// held it when it was an image of its own.
fn wrap_diffuse(layer: f32, ndotl: f32, radius_mm: f32) -> vec3<f32> {
    let row = layer * SSS_ROWS + curvature_row(radius_mm) + 0.5;
    let x = clamp((ndotl * 0.5 + 0.5) * SSS_COLS, 0.5, SSS_COLS - 0.5);
    let y = clamp(row, 0.5, SSS_ROWS * LAYERS - 0.5);
    return textureSampleLevel(flesh_luts, flesh_sampler, vec2<f32>(x / LUTS_W, y / LUTS_H), 0.0).rgb;
}

// One row of the packed image at film fraction `u`, still in the sRGB its bytes are written in: the packed image is
// `Rgba8Unorm`, so the decode an `Rgba8UnormSrgb` image did in hardware is done in `srgb_to_linear`.
fn blood_row_encoded(u: f32, row: f32) -> vec3<f32> {
    let x = clamp(u * LUTS_W, 0.5, LUTS_W - 0.5);
    return textureSampleLevel(flesh_luts, flesh_sampler, vec2<f32>(x / LUTS_W, (row + 0.5) / LUTS_H), 0.0).rgb;
}

fn srgb_to_linear(e: vec3<f32>) -> vec3<f32> {
    return select(pow((e + 0.055) / 1.055, vec3<f32>(2.4)), e / 12.92, e <= vec3<f32>(0.04045));
}

// **One row `r` of the blood table at film fraction `u`, `dried` of the way through its colour walk** (`0` fresh, `1`
// dry), decoded. The walk is `bloodstain::dry::appearance_with_fresh`'s: fresh to the methaemoglobin stop by `AGE_KNOT`, on
// to hemichrome at the end, interpolated in encoded sRGB between the table's rows and the aged table's, as the CPU
// interpolates it. Fresh blood takes the first branch and is the colour it always was, to the bit.
fn blood_row(u: f32, r: f32, dried: f32) -> vec3<f32> {
    if dried <= 0.0 {
        return srgb_to_linear(blood_row_encoded(u, BLOOD_ROW0 + r));
    }
    let early = dried < AGE_KNOT;
    let row_a = select(BLOOD_AGED_ROW0 + r, BLOOD_ROW0 + r, early);
    let row_b = select(BLOOD_AGED_ROW0 + 4.0 + r, BLOOD_AGED_ROW0 + r, early);
    let f = select((dried - AGE_KNOT) / (1.0 - AGE_KNOT), dried / AGE_KNOT, early);
    return srgb_to_linear(mix(blood_row_encoded(u, row_a), blood_row_encoded(u, row_b), clamp(f, 0.0, 1.0)));
}

// The film's reflectance over `base`, from the black/white rows of the blood table, interpolated by
// the base per channel, `dried` of the way through its colour walk. Row pairs: arterial (0, 1), venous (2, 3); the
// saturation dial picks a mix.
fn blood_over(base: vec3<f32>, depth_mm: f32, dried: f32) -> vec3<f32> {
    let u = clamp(depth_mm / BLOOD_LUT_MAX_MM, 0.0, 1.0);
    let art_black = blood_row(u, 0.0, dried);
    let art_white = blood_row(u, 1.0, dried);
    let ven_black = blood_row(u, 2.0, dried);
    let ven_white = blood_row(u, 3.0, dried);
    // `wet.w` is SO₂; venous 0.75 → 0, arterial 0.97 → 1.
    let a = clamp((flesh.wet.w - 0.75) / 0.22, 0.0, 1.0);
    let black = mix(ven_black, art_black, a);
    let white = mix(ven_white, art_white, a);
    return black + (white - black) * base;
}

// **How dry the blood a fragment is on is**, `0` fresh to `1` dry: one minus the wetness byte, averaged over the texels the
// wet map's spline reads (the four by four round the fragment, by the spline's own weights) **that have blood in them**, read at
// the texels and not through the spline. The spline's own wetness falls to zero at the rim of every drop, where the bare
// texels beside it carry none, which would draw a brown ring on every fresh one; weighting by "has blood" is the
// drop's own age out to the edge it is drawn to, and is smooth where two ages meet. A dried texel has blood and no wetness,
// so it counts as dry. Anything drawn has blood in the spline's support, so the weight is never zero for it.
fn blood_dried(uv: vec2<f32>) -> f32 {
    let dims = vec2<i32>(textureDimensions(wet_map));
    let p = uv * vec2<f32>(dims) - 0.5;
    let f = fract(p);
    let at = vec2<i32>(floor(p));
    let wx = bspline(f.x);
    let wy = bspline(f.y);
    var wet = 0.0;
    var weight = 0.0;
    for (var dy = 0; dy < 4; dy = dy + 1) {
        for (var dx = 0; dx < 4; dx = dx + 1) {
            let t = textureLoad(wet_map, clamp(at + vec2<i32>(dx - 1, dy - 1), vec2<i32>(0), dims - vec2<i32>(1)), 0);
            let w = wx[dx] * wy[dy] * select(0.0, 1.0, t.r > 0.0);
            wet = wet + w * t.a;
            weight = weight + w;
        }
    }
    return 1.0 - wet / max(weight, 1.0e-6);
}

// **How a film dries in space** (`docs/research/drying-front.md`). A film is not one age: it dries at its contact line first and the front
// goes in (Smith, Nicloux & Brutin 2020: "the rim dries first. Then this drying front propagates towards the centre of the pool"), and the
// dried film crazes into plates ("These cracks follow the drying front and the separation between them depends on the thickness of the
// sample"). The canvas stays the authority and carries one age for a whole stamp; this reads the film's own extent, as `crater_light`
// reads a crater's, and draws the age that follows from it. Nothing here is read back, and no table, canvas or digest depends on it.
//
// **How much sooner the rim dries than the middle**: the age the rim is drawn at is `age * (1 + 2.5) - 2.5 * reach`, so at an age of a
// quarter the rim is at three quarters of its walk and the middle at its own, and at an age of one every part of the film is at the end of it,
// as the CPU's walk has it. This crate's own number.
const FRONT_LEAD: f32 = 2.5;
// **How far in from its edge a film's colour is ahead of its age**: 30 mm, a dried rim 1.5 to 2 cm wide on a pool 3.6 cm across at the
// half dry (Smith et al. 2020, Fig. 12). A film further from its edge than this dries as a whole. This crate's own number.
const FRONT_REACH_MM: f32 = 30.0;
// Under this age a film is fresh and the front is not looked for.
const FRONT_MIN_AGE: f32 = 0.02;
// **How fast the middle of a film goes through its colour walk, against the clock**: 0.4. Oxidation under a skin of dried blood is slow
// (oxygen has to get in), and the middle of a pool stays red and wet until the front reaches it (Smith et al. 2020, Fig. 12, S.IV: a wet red
// middle in a black rim). At an age of one every part of the film has walked the whole way. This crate's own number.
const INTERIOR_RATE: f32 = 0.4;
// **The crust at the rim of a dried deep film**: the outermost `CRUST_REACH` (0.4 of the front's reach, 12 mm) is darkened by `CRUST_DARK`
// (0.6 of its linear light), and takes the colour of the film's full depth, however thin the canvas's falloff draws it. A film of a pool's
// depth has its solids carried to its contact line as it dries (the coffee ring: Brutin et al. 2011), and the rims of Ref Q's strokes and of
// Smith et al.'s pool are the darkest part of each. Not on a thin film (a drop). This crate's own numbers.
const CRUST_REACH: f32 = 0.4;
const CRUST_DARK: f32 = 0.6;
// Six directions, each sampled seven times out to the reach.
const FRONT_DIRS: u32 = 6u;
const FRONT_STEPS: u32 = 7u;
// The distances from the edge of the directions are joined by a p-norm minimum (a smooth minimum: a hard one draws a star).
const FRONT_NORM_P: f32 = 4.0;
const FRONT_EPS: f32 = 0.05;

// **The cracks**: plates about 12 mm across (Smith et al. 2020, Fig. 12 has them 4 to 6 mm on a pool 5 cm across: the plates here are wider so
// that they read from a metre or two), 0.35 mm wide or a pixel, in a film whose drawn age is past `CRAQUELURE_ONSET` (`bloodstain::dry`'s own 0.72)
// and which is deeper than a fifth of the canvas's, in patches. Plates under `CRACK_MIN_PX` pixels across are not drawn and plates under six
// times that are faded. This crate's own numbers.
const CRACK_SPACING_MM: f32 = 12.0;
const CRACK_WIDTH_MM: f32 = 0.35;
const CRACK_ONSET: f32 = 0.72;
const CRACK_MIN_PX: f32 = 2.0;

// How far a film runs from `uv` along `dir`, `x`, as a share of `FRONT_REACH_MM`: `0` at once at its edge, `1` when it runs on past the
// reach; and `y`, the deepest the film is on the way. Seven samples of the wet map at steps that are close together near `uv` and far
// apart beyond; the run ends where the film is under half of its depth at `uv`, placed by interpolating between the two samples that
// bracket it.
fn film_run(uv: vec2<f32>, dir: vec2<f32>, reach_uv: f32, core: f32) -> vec2<f32> {
    var run = 1.0;
    var deepest = core;
    var prev_t = 0.0;
    var prev_f = 1.0;
    var found = false;
    for (var k = 0u; k < FRONT_STEPS; k = k + 1u) {
        let s = f32(k + 1u) / f32(FRONT_STEPS);
        let t = pow(s, 1.8);
        let c = textureSampleLevel(wet_map, flesh_sampler, uv + dir * (t * reach_uv), 0.0).r;
        let f = clamp(c / max(core, WET_EDGE), 0.0, 1.0);
        if !found && f < 0.5 {
            let q = (prev_f - 0.5) / max(prev_f - f, 1.0e-4);
            run = mix(prev_t, t, q);
            found = true;
        }
        if !found {
            deepest = max(deepest, c);
        }
        prev_t = t;
        prev_f = f;
    }
    return vec2<f32>(run, deepest);
}

// **How far from its edge a fragment of film is**, `x`: `0` at the edge to `1` at `FRONT_REACH_MM` and beyond, the nearest of six directions
// by a smooth minimum; and `y`, how deep the film is where it is deepest within the reach, which is the film's own depth where the fragment is at its
// edge and the canvas's falloff has thinned it (a spline's foot is a texel or two wide). `y` is over zero: it is also the flag that this was
// computed.
fn film_front(uv: vec2<f32>, core: f32, mm_per_uv: f32) -> vec2<f32> {
    let reach_uv = FRONT_REACH_MM / max(mm_per_uv, 1.0e-6);
    var norm = 0.0;
    var plateau = core;
    for (var i = 0u; i < FRONT_DIRS; i = i + 1u) {
        let a = f32(i) * (2.0 * PI / f32(FRONT_DIRS));
        let dir = vec2<f32>(cos(a), sin(a));
        let r = film_run(uv, dir, reach_uv, core);
        norm = norm + pow(r.x + FRONT_EPS, -FRONT_NORM_P);
        plateau = max(plateau, r.y);
    }
    // What the smooth minimum comes to when the film runs on in every direction, so that "far from any edge" is exactly `1`.
    let full = pow(f32(FRONT_DIRS) * pow(1.0 + FRONT_EPS, -FRONT_NORM_P), -1.0 / FRONT_NORM_P) - FRONT_EPS;
    let reach = clamp((pow(norm, -1.0 / FRONT_NORM_P) - FRONT_EPS) / full, 0.0, 1.0);
    return vec2<f32>(reach, max(plateau, 1.0e-3));
}

// A hash of a lattice point, two floats in `[0, 1]` (PCG, Jarzynski and Olano 2020): integer arithmetic, so the same on every GPU.
fn hash22(p: vec2<i32>) -> vec2<f32> {
    var v = vec2<u32>(bitcast<u32>(p.x), bitcast<u32>(p.y)) * 1664525u + vec2<u32>(1013904223u);
    v.x = v.x + v.y * 1664525u;
    v.y = v.y + v.x * 1664525u;
    v = v ^ (v >> vec2<u32>(16u));
    v.x = v.x + v.y * 1664525u;
    v.y = v.y + v.x * 1664525u;
    v = v ^ (v >> vec2<u32>(16u));
    return vec2<f32>(v & vec2<u32>(0xFFFFu)) / 65535.0;
}

// `x`: how far `q` is from the nearest border between two cells of a jittered lattice, in cells; `y`: a number in `[0, 1]` that belongs to
// that border and not to `q`, the same on both sides of it.
fn crack_cells(q: vec2<f32>) -> vec2<f32> {
    let n = vec2<i32>(floor(q));
    let f = q - floor(q);
    var near_d = 8.0;
    var near_r = vec2<f32>(0.0);
    var near_id = vec2<i32>(0);
    for (var j = -1; j <= 1; j = j + 1) {
        for (var i = -1; i <= 1; i = i + 1) {
            let g = vec2<i32>(i, j);
            let r = vec2<f32>(g) + 0.1 + 0.8 * hash22(n + g) - f;
            let d = dot(r, r);
            if d < near_d {
                near_d = d;
                near_r = r;
                near_id = n + g;
            }
        }
    }
    var edge = 8.0;
    var own = 1.0;
    for (var j = -1; j <= 1; j = j + 1) {
        for (var i = -1; i <= 1; i = i + 1) {
            let g = near_id + vec2<i32>(i, j);
            let r = vec2<f32>(g - n) + 0.1 + 0.8 * hash22(g) - f;
            let diff = r - near_r;
            let len2 = dot(diff, diff);
            if len2 > 1.0e-5 {
                let d = dot(0.5 * (near_r + r), diff) * inverseSqrt(len2);
                if d < edge {
                    edge = d;
                    own = min(hash22(near_id).x, hash22(g).x);
                }
            }
        }
    }
    return vec2<f32>(edge, own);
}

// A smooth value noise in `[0, 1]` over the lattice's points: which patches of a film have cracked.
fn patch_noise(p: vec2<f32>) -> f32 {
    let i = vec2<i32>(floor(p));
    let f = p - floor(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hash22(i).x;
    let b = hash22(i + vec2<i32>(1, 0)).x;
    let c = hash22(i + vec2<i32>(0, 1)).x;
    let d = hash22(i + vec2<i32>(1, 1)).x;
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// **How much of a crack a fragment of a dried film is on**, `[0, 1]`. `age` is the age it is drawn at, `front` what `film_front` found. A dried
// film of a pool's depth crazes into plates (the cracks "follow the drying front", and "the separation between them depends on the
// thickness of the sample": Smith et al. 2020), so the cracks are the borders of a jittered lattice whose cell is `CRACK_SPACING_MM` wide in a
// film of a pool's depth and wider in a deeper one, in patches, in mm on the surface so that the size does not depend on the canvas.
fn dried_crack(uv: vec2<f32>, front: vec2<f32>, age: f32, mm_per_uv: f32, mm_per_px: f32) -> f32 {
    let cell = CRACK_SPACING_MM;
    let gate = smoothstep(CRACK_ONSET, CRACK_ONSET + 0.12, age)
        * smoothstep(0.2, 0.4, front.y)
        * smoothstep(CRACK_MIN_PX, 6.0 * CRACK_MIN_PX, cell / max(mm_per_px, 1.0e-6));
    if gate <= 0.0 {
        return 0.0;
    }
    let here = uv * mm_per_uv;
    let v = crack_cells(here / cell);
    let half_w = 0.5 * max(CRACK_WIDTH_MM, 0.7 * mm_per_px) / cell;
    let aa = 0.5 * mm_per_px / cell;
    let line = 1.0 - smoothstep(half_w - aa, half_w + aa, v.x);
    // A film cracks in patches, and not every border of a plate is a crack.
    let p = here / (cell * 9.0);
    let cracked = smoothstep(0.45, 0.65, 0.65 * patch_noise(p) + 0.35 * patch_noise(p * 2.7 + vec2<f32>(17.0, 5.0)));
    let present = smoothstep(0.15, 0.3, v.y);
    return line * cracked * present * gate;
}

// **The normal of a surface lifted above the one at `N`**, given how fast the lift changes across the
// screen (`dh`, world units per pixel in x and y), so it needs no tangents (Mikkelsen, "Bump Mapping
// Unparametrized Surfaces on the GPU", 2010). Derivatives: call it only in uniform control flow.
fn lifted_normal(N: vec3<f32>, P: vec3<f32>, dh: vec2<f32>) -> vec3<f32> {
    let dpx = dpdx(P);
    let dpy = dpdy(P);
    let r1 = cross(dpy, N);
    let r2 = cross(N, dpx);
    let det = dot(dpx, r1);
    let grad = sign(det) * (dh.x * r1 + dh.y * r2);
    let n = normalize(abs(det) * N - grad);
    let c = dot(n, N);
    if c >= FILM_MAX_TILT_COS {
        return n;
    }
    // Turned toward `N` until it leans no further than the cap allows.
    let side = normalize(n - N * c);
    return N * FILM_MAX_TILT_COS + side * sqrt(1.0 - FILM_MAX_TILT_COS * FILM_MAX_TILT_COS);
}

// **How many directions and how far a crater's floor looks for its walls.** Eight directions, each
// out to 1, 3, 7 and 15 texels: on the mannequin's 256-texel skins, 3.6 mm a texel, that is 54 mm,
// past the rim of a 30 mm wound.
const CAVITY_DIRS: u32 = 8u;
const CAVITY_STEPS: u32 = 4u;

// **How much of the room a point in a crater sees**, `[0, 1]`: one minus the mean, over
// `CAVITY_DIRS` directions, of the sine of the highest wall it sees that way. In a section across a
// hollow, the cosine-weighted share of a point's view that clears both walls is
// `1 − (sin h₁ + sin h₂) / 2` for their elevations `h₁, h₂` (the crossed-strings result a wound bed's
// lips and floor are lit by), and the mean over directions is that, all the way round. The walls are
// the flaymap's own depth, `depth_mm` here against the depth at each tap, `mm_per_uv` the canvas's
// size on the body. It leaves out the light the walls throw back.
fn crater_light(uv: vec2<f32>, depth_mm: f32, mm_per_uv: f32) -> f32 {
    let size = vec2<f32>(textureDimensions(pbr_bindings::metallic_roughness_texture));
    let mm_per_texel = mm_per_uv / max(size.x, 1.0);
    var seen = 0.0;
    for (var i = 0u; i < CAVITY_DIRS; i = i + 1u) {
        let a = (f32(i) + 0.5) * (2.0 * PI / f32(CAVITY_DIRS));
        let dir = vec2<f32>(cos(a), sin(a));
        var wall = 0.0;
        for (var k = 0u; k < CAVITY_STEPS; k = k + 1u) {
            let texels = f32((2u << k) - 1u);
            let tap = textureSampleLevel(pbr_bindings::metallic_roughness_texture, pbr_bindings::metallic_roughness_sampler, uv + dir * texels / size, 0.0).r;
            let rise = depth_mm - tap * flesh.cut.y;
            let run = texels * mm_per_texel;
            wall = max(wall, rise / max(sqrt(rise * rise + run * run), 1.0e-6));
        }
        seen += wall;
    }
    return clamp(1.0 - seen / f32(CAVITY_DIRS), 0.0, 1.0);
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let mode = flesh.sss.z;
    let flags = u32(flesh.sss.w);

    // ---- The canvases: what is wet, what is peeled. ------------------------------------------
    var amount = 0.0;      // film depth byte, 0..1
    var wetness = 0.0;     // fresh 1 → dry 0
    var dried = 0.0;       // the blood's own age: fresh 0 → dry 1
    var depth_frac = 0.0;  // depth over the layer span
    var peeled = 0.0;      // 1 where the skin is gone
    var film_n = pbr_input.N;            // the normal of a film's own surface
    var film_cn = pbr_input.clearcoat_N; // and of the coat on it
    var film_dh = vec2<f32>(0.0);        // how fast the film's surface rises across the screen
    var flay_mm_per_uv = 0.0;            // the flaymap's size on the body, for a crater's walls
    var mm_per_px = 0.0;                 // how big a pixel is on the surface, and
    var mm_per_uv = 0.0;                 // how big the canvas is, for the drying front: both zero where it cannot be taken (the prepass)
    var front = vec2<f32>(0.0);          // where an aged film is in its drying: `film_front`, `y` zero when not looked for
@if(VERTEX_UVS_A) {
    if (flags & FLAG_WET) != 0u {
        // Smooth inside, and its rim drawn crisp at `WET_EDGE`: a film's edge is a line, and the
        // B-spline alone would leave a soft skirt a texel wide. `fwidth` is the smoothed amount's
        // change over one pixel, so the rim is a pixel wide however far the canvas is magnified, and
        // anti-aliased rather than stepped where it is minified.
        let w = wet_smooth(in.uv);
        let aa = max(fwidth(w.r), 1.0e-4);
        amount = w.r * clamp((w.r - WET_EDGE) / aa + 0.5, 0.0, 1.0);
        wetness = w.a;
@if(!PREPASS_PIPELINE) {
        // Taken here, in uniform control flow, for the drying front and its cracks: a derivative cannot be taken under `amount > 0.0`.
        mm_per_px = length(fwidth(pbr_input.world_position.xyz)) * flesh.sss.y;
        mm_per_uv = mm_per_px / max(length(fwidth(in.uv)), 1.0e-9);
}
        if amount > 0.0 {
            dried = blood_dried(in.uv);
            if dried > FRONT_MIN_AGE && mode != MODE_CLOTH && mm_per_uv > 0.0 {
                front = film_front(in.uv, w.r, mm_per_uv);
            }
        }
@if(!PREPASS_PIPELINE) {
        // **A film has a surface.** Its depth, smooth to its second derivative, is a height above
        // what it lies on, and that height tilts the normal: a pool's edge curves down to the floor
        // and catches the light as a meniscus does, a run stands up off the skin, and a pool's
        // unevenness breaks a light's mirror image into a glint. Before 0.5.47 a film was flat however
        // deep, a decal of one colour to its rim, and a pool at a light's mirror angle was one white
        // sticker. A weave holds its blood between the yarns, so cloth stays flat. Leveling a pool
        // above a quarter of its depth, as a liquid at rest would be, was tried and made that sticker
        // again.
        //
        // The slope is the spline's own, a central difference a quarter of a texel either way, carried
        // to the screen by the UV's derivatives, which are constant across a triangle; taken by `dpdx`
        // of the height instead it was a step per 2×2 pixels, and a run on a distant arm glittered. It
        // fades out where a texel is under a pixel and a surface that fine cannot be drawn.
        if mode != MODE_CLOTH {
            let size = vec2<f32>(textureDimensions(wet_map));
            let du = vec2<f32>(0.25 / size.x, 0.0);
            let dv = vec2<f32>(0.0, 0.25 / size.y);
            let slope = vec2<f32>(
                (wet_smooth(in.uv + du).r - wet_smooth(in.uv - du).r) / (2.0 * du.x),
                (wet_smooth(in.uv + dv).r - wet_smooth(in.uv - dv).r) / (2.0 * dv.y),
            );
            let uv_x = dpdx(in.uv);
            let uv_y = dpdy(in.uv);
            let px_per_texel = 1.0 / max(max(length(uv_x * size), length(uv_y * size)), 1.0e-6);
            let lift = flesh.wet.z / max(flesh.sss.y, 1.0e-6) * FILM_RELIEF * smoothstep(0.35, 1.0, px_per_texel);
            // A film only a few texels across lies nearly flat: see `FILM_SMALL_RELIEF`.
            let reach = FILM_SPAN_TEXELS / size;
            let around = 0.25 * (
                textureSample(wet_map, flesh_sampler, in.uv + reach).r
                + textureSample(wet_map, flesh_sampler, in.uv - reach).r
                + textureSample(wet_map, flesh_sampler, in.uv + vec2<f32>(reach.x, -reach.y)).r
                + textureSample(wet_map, flesh_sampler, in.uv + vec2<f32>(-reach.x, reach.y)).r
            );
            let span = mix(FILM_SMALL_RELIEF, 1.0, smoothstep(FILM_SPAN_LOW, FILM_SPAN_HIGH, around / max(w.r, 0.05)));
            film_dh = vec2<f32>(dot(slope, uv_x), dot(slope, uv_y)) * lift * span;
        }
}
    }
    if (flags & FLAG_FLAY) != 0u {
        // The canvas's roughness image is in the base's metallic-roughness slot (see
        // `FleshTables::material`): `R` the depth over the span, `A` whether the texel is peeled.
        // `pbr_bindings` is the non-bindless layout, which is the only one this material has:
        // `ExtendedMaterial` goes bindless only when its extension does, and `FleshExtension` does not.
        let f = textureSample(pbr_bindings::metallic_roughness_texture, pbr_bindings::metallic_roughness_sampler, in.uv);
        depth_frac = f.r;
        peeled = f.a;
        // The canvas's albedo is the flat colour it was dressed with wherever the skin is intact.
        // Swap that for the skin as authored (texture, factor, vertex colours) by exactly the intact
        // share of the texel. A peeled texel keeps the canvas's tissue, and an untextured skin, whose
        // authored colour *is* the flat one, comes out as the canvas drew it.
        let canvas = textureSample(flay_map, flesh_sampler, in.uv).rgb;
        let flat = pbr_bindings::material.base_color.rgb;
        let authored = pbr_input.material.base_color.rgb;
        pbr_input.material.base_color = vec4<f32>(canvas + (1.0 - peeled) * (authored - flat), pbr_input.material.base_color.a);
@if(!PREPASS_PIPELINE) {
        // The canvas's size on the body, millimetres per UV, from how far the surface and its UV move
        // across a pixel. Taken here, where the flags are uniform: a derivative cannot be taken in
        // the per-pixel branch that uses it.
        flay_mm_per_uv = length(fwidth(pbr_input.world_position.xyz)) / max(length(fwidth(in.uv)), 1.0e-9) * flesh.sss.y;
}
    }
    // What lies under intact skin: a bruise's chromophores or a burn's eschar modulate the skin's
    // own colour (the map is a ratio, linear), and go with the skin when it is peeled. On cloth the
    // same slot holds the holes shot through it: `rgb` the ratio of the ring around each, shown by
    // twice `a`, and the cloth gone wherever `a` is over one half — the alpha the mask cuts on.
    if (flags & FLAG_DERMIS) != 0u {
        let d = textureSample(dermis_map, flesh_sampler, in.uv);
        if mode == MODE_CLOTH {
            let ratio = mix(vec3<f32>(1.0), d.rgb, clamp(d.a * 2.0, 0.0, 1.0));
            let kept = 1.0 - step(0.5, d.a);
            pbr_input.material.base_color = vec4<f32>(pbr_input.material.base_color.rgb * ratio, pbr_input.material.base_color.a * kept);
        } else {
            let show = d.a * (1.0 - peeled);
            let ratio = mix(vec3<f32>(1.0), d.rgb, show);
            pbr_input.material.base_color = vec4<f32>(pbr_input.material.base_color.rgb * ratio, pbr_input.material.base_color.a);
        }
    }
@if(!PREPASS_PIPELINE) {
    // **Blood in a crater lies level.** It fills a hollow the skin was peeled from, so the relief
    // is a film's on intact skin and none of it shows over a peeled texel; lifted there, the clots of
    // a crater seen close stood up as cobbles with dark crevices between them.
    if (flags & FLAG_WET) != 0u && mode != MODE_CLOTH {
        let dh = film_dh * (1.0 - peeled);
        film_n = lifted_normal(pbr_input.N, pbr_input.world_position.xyz, dh);
        film_cn = lifted_normal(pbr_input.clearcoat_N, pbr_input.world_position.xyz, dh);
    }
}
}
@if(VERTEX_UVS_B) {
    if mode == MODE_CAP {
        depth_frac = in.uv_b.x;
    }
}
    // ---- Exposed tissue, wet with its own blood (`FleshParams::cut`). ------------------------
    // A cut face, and a crater's walls where the skin is peeled. Thickest where the vessels are, in
    // the muscle, by the tissue's own depth below the skin (`UV_1.x` on a cap, the flaymap's depth on
    // a canvas). A cap keeps a quarter of it over the fat at its rim, so a piece reads as meat with a
    // paler edge and the bands still show; a crater's graze keeps `GRAZE_FILM` of it, the ooze of a
    // fresh abrasion, so the dermis reads raw and the fat under it reddens, and the deep wound is
    // dark with blood, where bare it was a ring of yellow fat. Its liquid shines. A value noise over
    // `UV_1` was tried first, for patches, and on the preset's pieces it drew stripes across each
    // face, not patches.
    var cut_coat = 0.0;
    var exposed = 0.0;
    var rim = 0.0;
@if(VERTEX_UVS_B) {
    if mode == MODE_CAP {
        exposed = 1.0;
        rim = 0.25;
    }
}
    if mode == MODE_CANVAS {
        exposed = peeled;
        rim = GRAZE_FILM * smoothstep(0.0, GRAZE_RAMP, depth_frac);
    }
    if flesh.cut.x > 0.0 && exposed > 0.0 {
        let thick = smoothstep(0.08, 0.5, depth_frac);
        let film_mm = flesh.cut.x * mix(rim, 1.0, thick);
        // How wet it shines: a half where a cap's rim film lies, a little on a graze, all of it deep.
        let shine = mix(2.0 * rim, 1.0, thick) * exposed;
        let tissue = pbr_input.material.base_color.rgb;
        pbr_input.material.base_color = vec4<f32>(mix(tissue, blood_over(tissue, film_mm, 0.0), exposed), pbr_input.material.base_color.a);
        pbr_input.material.perceptual_roughness = mix(pbr_input.material.perceptual_roughness, 0.45, shine);
        cut_coat = flesh.wet.x * shine;
    }
    let layer = layer_of(depth_frac);

    // ---- Blood: composite the film over whatever is under it, on the GPU. -------------------
    // The wetmap's own albedo image already carries this film over a grey; reading the amount byte
    // and compositing here instead keeps the hue of the base — skin, a peeled tissue, a weave.
    // **How much liquid there is to shine**, as a share of enough to mirror: nothing under two per cent
    // of the film's depth, all of it from a quarter of it up. A smear a few microns deep follows the
    // surface it lies on and only a pool smooths it. Every film used to count as a pool, and a wet
    // surface went to a roughness of 0.25 and a coat of 1.0 at the first texel of blood. A flat pool in
    // a light's mirror angle then clipped to white, so 0.5.23 took the gloss to 0.45. Since 0.5.47 a
    // film's surface has relief, which breaks that mirror into a glint, and the gloss is 0.3. A liquid's
    // own 0.1 was tried: with no environment to reflect, the sheen that says "wet" shrank to a speck
    // and runs and pools read as matte paint.
    let pool = smoothstep(0.02, 0.25, amount);
    // Blood in a weave is held between the yarns rather than stood on them, so a soaked shirt is
    // darker and a little smoother, never a mirror: 0.7 and a third of the coat, where a film is 0.3.
    let weave = mode == MODE_CLOTH;
    let wet_rough = select(0.3, 0.7, weave);
    if amount > 0.0 {
        let depth_mm = amount * flesh.wet.z;
        let base = pbr_input.material.base_color.rgb;
        // **The age the film is drawn at is its own, not its stamp's** (`film_front`): the rim is ahead of the middle, and where it is
        // ahead it is matte. Under `FRONT_MIN_AGE`, in the prepass and on cloth it is the canvas's age, as it was.
        var age = dried;
        var crack = 0.0;
        var crust = 0.0;
        var colour_mm = depth_mm;
        if front.y > 0.0 {
            age = max(dried * INTERIOR_RATE, clamp(dried * (1.0 + FRONT_LEAD) - FRONT_LEAD * front.x, 0.0, 1.0));
            wetness = wetness * (1.0 - smoothstep(0.0, 0.5, age - dried));
            crack = dried_crack(in.uv, front, age, mm_per_uv, mm_per_px);
            // The dried rim of a deep film is a crust, darker than the film it bounds (the coffee ring).
            crust = smoothstep(0.5, 1.0, age) * (1.0 - smoothstep(0.0, CRUST_REACH, front.x)) * smoothstep(0.1, 0.3, front.y);
            // A dried rim is as deep as the film it bounds, however thin the canvas's falloff draws it (the contact line is pinned and the
            // solids are carried out to it), so it takes the colour of the deep film: the black of a thick crust, not the grey of a thin one.
            colour_mm = mix(depth_mm, max(depth_mm, front.y * flesh.wet.z), smoothstep(0.3, 0.8, age) * smoothstep(0.1, 0.3, front.y));
        }
        // A crack shows what the blood lies on.
        let blood = mix(blood_over(base, colour_mm, age), base * 0.6, 0.4 * crack) * (1.0 - CRUST_DARK * crust);
        pbr_input.material.base_color = vec4<f32>(blood, pbr_input.material.base_color.a);
        // A wet film smooths whatever it lies on; a dried one is matte again.
        pbr_input.material.perceptual_roughness = mix(pbr_input.material.perceptual_roughness, wet_rough, wetness * pool);
        pbr_input.N = film_n;
        pbr_input.clearcoat_N = film_cn;
    }

    // ---- The wet clear coat: a specular layer of liquid above the diffuse colour. --------------
    let coat = max(wetness * flesh.wet.x * pool * select(1.0, 1.0 / 3.0, weave), cut_coat);
    pbr_input.material.clearcoat = coat;
    pbr_input.material.clearcoat_perceptual_roughness = flesh.wet.y;

    pbr_input.material.base_color = alpha_discard(pbr_input.material.flags, pbr_input.material.alpha_cutoff, pbr_input.material.base_color);

@if(PREPASS_PIPELINE)
    let out = deferred_output(in, pbr_input);
@if(!PREPASS_PIPELINE)
    var out: FragmentOutput;
@if(!PREPASS_PIPELINE) {
    out.color = apply_pbr_lighting(pbr_input);

    // ---- Subsurface: the wrap the diffusion profile adds past the terminator. -----------------
    let strength = flesh.sss.x;
    if strength > 0.0 && mode < MODE_CLOTH {
        let N = pbr_input.N;
        let P = pbr_input.world_position.xyz;
        // Curvature from screen-space derivatives (Penner 2011): |dN| / |dP| is 1/r in mesh units.
        let dn = length(fwidth(N));
        let dp = max(length(fwidth(P)), 1.0e-6);
        let radius_units = dp / max(dn, 1.0e-6);
        let radius_mm = radius_units * flesh.sss.y;

        let diffuse_color = pbr_input.material.base_color.rgb * (1.0 - pbr_input.material.metallic);
        var extra = vec3<f32>(0.0);
        let n_directional_lights = view_bindings::lights.n_directional_lights;
        for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
            let light = &view_bindings::lights.directional_lights[i];
            let L = (*light).direction_to_light;
            let ndotl = dot(N, L);
            var shadow: f32 = 1.0;
            if ((pbr_input.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                    && ((*light).flags & mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u) {
                let view_z = dot(vec4<f32>(
                    view_bindings::view.view_from_world[0].z,
                    view_bindings::view.view_from_world[1].z,
                    view_bindings::view.view_from_world[2].z,
                    view_bindings::view.view_from_world[3].z
                ), pbr_input.world_position);
                shadow = shadows::fetch_directional_shadow(i, pbr_input.world_position, pbr_input.world_normal, view_z, pbr_input.frag_coord.xy);
            }
            let wrap = wrap_diffuse(layer, ndotl, radius_mm);
            let lambert = vec3<f32>(max(ndotl, 0.0));
            extra += diffuse_color * (wrap - lambert) * (*light).color.rgb * shadow / PI;
        }
        // No separate ambient term, deliberately: under uniform ambient light a convex surface's
        // subsurface response integrates to its total diffuse reflectance, and that reflectance *is*
        // the base colour the ambient term already multiplies. Adding the profile's hue on top
        // counted it twice and turned skin in shadow terracotta (measured, 0.4.0).
        // `apply_pbr_lighting` scales every light term by the view's exposure at the end; these are
        // light terms too.
        out.color = vec4<f32>(out.color.rgb + extra * strength * view_bindings::view.exposure, out.color.a);
    }

@if(VERTEX_UVS_A) {
    // ---- A crater is lit through its mouth. -------------------------------------------------
    // Every light term, the coat's and the subsurface's with them: a crater's floor sees the room
    // only between its walls (`crater_light`). Drawn as open skin, a wound pooled with blood was a
    // flat red decal whatever its depth.
    if (flags & FLAG_FLAY) != 0u && mode == MODE_CANVAS && depth_frac > 0.0 {
        let lit = crater_light(in.uv, depth_frac * flesh.cut.y, flay_mm_per_uv);
        out.color = vec4<f32>(out.color.rgb * lit, out.color.a);
    }
}

    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
}

    return out;
}
