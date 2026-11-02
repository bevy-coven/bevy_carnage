//! **Fluids that are not blood.**
//!
//! [`crate::bloodstain::spectral`] computes a film's colour from an absorbing, scattering liquid over a substrate, and
//! nothing in the layer solution is blood's: it is Kubelka and Munk's two-flux film, `K = 2 μa` and `S = ¾ μs'` per
//! wavelength, then the CIE observer under D65. A **fluid** is a different `μa(λ)` and `μs'` in that same model, so a
//! film of it is thin and tinted over a pale substrate, deep and opaque in a pool, and the colour follows with no
//! authored swatch.
//!
//! [`Fluid::Blood`] is the model the crate has always had (Bosschaart et al.'s measured whole-blood optics, the
//! oxygen-saturation dial on [`crate::bloodstain::Film`], the three-stop haemoglobin age walk of
//! [`crate::bloodstain::dry`]) and every function here answers it **to the bit** by calling the function it has always
//! called. The four named pigments and [`Fluid::Custom`] are the others, and what each is made of is in
//! `docs/research/fluids.md`:
//!
//! | fluid | what it is | its colour comes from |
//! |---|---|---|
//! | [`Fluid::Ichor`] | black oil | melanin: Jacques's power law (`μa = 1.70×10¹² λ^-3.48 cm⁻¹` for a melanosome, flattened here to `λ^-2.6` so the black is not a brown), so thin is brown and thick is black |
//! | [`Fluid::Green`] | green blood | biliverdin (the Prasinohaema skinks'): a violet Soret band and a red Q band, green between them |
//! | [`Fluid::Blue`] | blue blood | oxyhaemocyanin (octopus, horseshoe crab): copper-peroxide charge-transfer bands near 486, 570 and 625 nm |
//! | [`Fluid::Milk`] | white blood | scattering and almost no absorption: Aernouts's and Stocker's milk, 1-8 × 10⁻³ mm⁻¹ |
//!
//! # What a fluid changes, and what it does not
//!
//! Its **optics** and its **ageing**: the fresh colour of a film of a given thickness over a given substrate, and how
//! that colour dulls as the fluid sets ([`Pigment::dried`], [`Pigment::dries`]). It never reaches the simulation: where a
//! droplet lands, how a run runs, when a wound clots and every digest are the same under every fluid, because this is a
//! colour the CPU hands to the canvases and the GPU's tables and nothing a hash reads.
//!
//! The ageing of a pigment is **this crate's own** (no paper says how glow-stick fluid, condensed milk or an invented
//! ichor dries); it is monotone, bounded and tuned against the note's reference set, and says so on [`Pigment`].

use crate::bloodstain::m;
use crate::bloodstain::spectral::{
    self, Film, SAMPLES, TABLE, encode, kubelka_munk, xyz, xyz_to_linear_srgb,
};

/// Absorption bands a [`Pigment`] may carry. Three is what the haemocyanin's 486, 570 and 625 nm bands need and what
/// the biliverdin's two need with room to spare.
pub const BANDS: usize = 3;

/// **One Gaussian absorption band**: `mua` mm⁻¹ at its centre, falling with a standard deviation of `width_nm`.
///
/// A band with no `mua` is not there, so a pigment with fewer than [`BANDS`] fills the rest with [`Band::NONE`].
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Band {
    /// Centre, nm.
    pub nm: f32,
    /// One standard deviation of the band's width, nm.
    pub width_nm: f32,
    /// Absorption coefficient at the centre, mm⁻¹.
    pub mua: f32,
}

impl Band {
    /// A band that absorbs nothing.
    pub const NONE: Band = Band {
        nm: 550.0,
        width_nm: 1.0,
        mua: 0.0,
    };

    /// The band's absorption at `nm`: zero when it has none, and nothing for a width that is not positive and finite.
    fn at(&self, nm: f32) -> f32 {
        if !(self.mua > 0.0)
            || !(self.width_nm > 0.0)
            || !self.width_nm.is_finite()
            || !self.nm.is_finite()
        {
            return 0.0;
        }
        let z = (nm - self.nm) / self.width_nm;
        self.mua * m::exp(-0.5 * z * z)
    }
}

/// **A turbid liquid that is a pigment's colour**: an absorption spectrum, a flat reduced scattering, and how it dries.
///
/// ```text
/// μa(λ) = floor + continuum · (λ / 500 nm)^-exponent + Σ bands  peak · exp(-½ ((λ - centre) / width)²)
/// K = 2 μa        S = ¾ μs'
/// ```
///
/// **Every number a named pigment carries is this crate's own**, tuned against `docs/research/fluids.md`'s reference set
/// and standing on the optics that note cites; nothing here claims to be a measurement of a liquid a film set used.
/// A [`Fluid::Custom`] game authors the same fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Pigment {
    /// A flat absorption under everything else, mm⁻¹. Milk's is Stocker's `1e-3` to `8e-3`.
    pub floor: f32,
    /// A power-law continuum's absorption at 500 nm, mm⁻¹: melanin's, which falls with wavelength as `λ^-exponent`. `0` for none.
    pub continuum: f32,
    /// The continuum's exponent. Jacques's melanosome fit is `3.48`; [`Fluid::Ichor`] uses `2.6`, which greys the black.
    pub exponent: f32,
    /// Up to [`BANDS`] absorption bands.
    pub bands: [Band; BANDS],
    /// **Reduced scattering** `μs'`, mm⁻¹, flat across the visible. Whole milk is of the order of `6`.
    pub scatter: f32,
    /// **The multiplier on each channel of the fresh colour, once the fluid has set**: a fluid keeps its hue and loses
    /// its freshness, and a thin film stays lighter than a thick one because it is the fresh colour that is scaled.
    /// `[1, 1, 1]` is a fluid that does not change. This crate's own.
    pub dried: [f32; 3],
    /// **How far the fluid sets**, `[0, 1]`: `1` is blood's pace, `0` a fluid that stays wet and glossy for ever (an oil).
    /// It scales both the colour's walk and the gloss's fall. This crate's own.
    pub dries: f32,
}

impl Pigment {
    /// The absorption coefficient at `nm`, mm⁻¹.
    pub fn mua(&self, nm: f32) -> f32 {
        let mut a = self.floor.max(0.0);
        if self.continuum > 0.0 && self.exponent.is_finite() && nm > 0.0 {
            a += self.continuum * m::powf(nm / 500.0, -self.exponent);
        }
        for band in &self.bands {
            a += band.at(nm);
        }
        a
    }

    /// The reflectance spectrum of a film of this pigment, one value per [`TABLE`] row. A film's oxygen saturation means
    /// nothing here and is ignored.
    pub fn reflectance(&self, film: &Film) -> [f32; SAMPLES] {
        let rg = film.substrate.clamp(0.0, 1.0);
        let d = film.thickness_mm.max(0.0);
        let s = 0.75 * self.scatter.max(0.0);
        let mut out = [0.0f32; SAMPLES];
        for (o, row) in out.iter_mut().zip(TABLE.iter()) {
            *o = kubelka_munk(2.0 * self.mua(row.nm as f32), s, d, rg);
        }
        out
    }
}

/// **What the preset's fluid is.** `Blood` is what the crate has always drawn; the rest are the optics of other liquids
/// in the same film model. Append-only: a variant's place is authored in config.
///
/// A [`Pigment`] that is not [`Fluid::Custom`] is a constant (see [`Fluid::pigment`]).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fluid {
    /// **Blood.** The Bosschaart table, the oxygen-saturation dial and the haemoglobin age walk. The default.
    #[default]
    Blood,
    /// **Black oil**: melanin's power-law absorption in a scattering carrier. Brown where it is thin, black where it is thick,
    /// and it **stays glossy**: Carmageddon's German "oil", *Evil Dead II*'s black.
    Ichor,
    /// **Green blood**: biliverdin's two bands with green between them, the pigment of the skinks whose blood is green and
    /// the hue of *Re-Animator*'s reagent and *Evil Dead II*'s goo.
    Green,
    /// **Blue blood**: oxyhaemocyanin's copper-peroxide bands, the blood of an octopus.
    Blue,
    /// **White blood**: milk, all scattering and almost no absorption, translucent where it is thin and opaque cream where it is
    /// thick: Ash's in *Alien*.
    Milk,
    /// A pigment the game describes.
    Custom(Pigment),
}

/// Melanin's ichor. See [`Fluid::Ichor`]. **This crate's own numbers**; the exponent is Jacques's `3.48` flattened to `2.6`.
const ICHOR: Pigment = Pigment {
    floor: 0.02,
    continuum: 25.0,
    exponent: 2.6,
    bands: [Band::NONE; BANDS],
    scatter: 1.0,
    dried: [0.9, 0.86, 0.8],
    dries: 0.3,
};

/// Biliverdin's green. See [`Fluid::Green`]. **This crate's own numbers**; the band positions are the literature's.
const GREEN: Pigment = Pigment {
    floor: 0.02,
    continuum: 0.0,
    exponent: 3.48,
    bands: [
        Band {
            nm: 380.0,
            width_nm: 35.0,
            mua: 60.0,
        },
        Band {
            nm: 435.0,
            width_nm: 30.0,
            mua: 10.0,
        },
        Band {
            nm: 700.0,
            width_nm: 35.0,
            mua: 40.0,
        },
    ],
    scatter: 3.0,
    dried: [0.38, 0.42, 0.22],
    dries: 0.9,
};

/// Oxyhaemocyanin's blue. See [`Fluid::Blue`]. **This crate's own numbers**; the band positions are the literature's.
const BLUE: Pigment = Pigment {
    floor: 0.03,
    continuum: 0.0,
    exponent: 3.48,
    bands: [
        Band {
            nm: 585.0,
            width_nm: 30.0,
            mua: 14.0,
        },
        Band {
            nm: 486.0,
            width_nm: 25.0,
            mua: 1.0,
        },
        Band {
            nm: 645.0,
            width_nm: 40.0,
            mua: 14.0,
        },
    ],
    scatter: 3.0,
    dried: [0.45, 0.5, 0.55],
    dries: 0.9,
};

/// Milk. See [`Fluid::Milk`]. **This crate's own numbers** on Stocker's absorption and Aernouts's scattering.
const MILK: Pigment = Pigment {
    floor: 0.004,
    continuum: 0.0,
    exponent: 3.48,
    bands: [
        Band {
            nm: 450.0,
            width_nm: 30.0,
            mua: 0.02,
        },
        Band::NONE,
        Band::NONE,
    ],
    scatter: 6.0,
    dried: [0.92, 0.8, 0.52],
    dries: 1.0,
};

impl Fluid {
    /// The pigment this fluid is, or `None` for [`Fluid::Blood`].
    pub fn pigment(&self) -> Option<Pigment> {
        match self {
            Fluid::Blood => None,
            Fluid::Ichor => Some(ICHOR),
            Fluid::Green => Some(GREEN),
            Fluid::Blue => Some(BLUE),
            Fluid::Milk => Some(MILK),
            Fluid::Custom(p) => Some(*p),
        }
    }

    /// **The reflectance spectrum of a film of this fluid**, one value per [`TABLE`] row. For blood it is
    /// [`spectral::reflectance`] itself.
    pub fn reflectance(&self, film: &Film) -> [f32; SAMPLES] {
        match self.pigment() {
            None => spectral::reflectance(film),
            Some(p) => p.reflectance(film),
        }
    }

    /// The colour of a film of this fluid, linear sRGB in `[0, 1]`. For blood, [`spectral::linear_srgb`] to the bit.
    pub fn linear_srgb(&self, film: &Film) -> [f32; 3] {
        xyz_to_linear_srgb(xyz(&self.reflectance(film)))
    }

    /// **The colour of a film of this fluid**, encoded sRGB in `[0, 1]`. For blood, [`spectral::srgb`] to the bit.
    pub fn srgb(&self, film: &Film) -> [f32; 3] {
        let [r, g, b] = self.linear_srgb(film);
        [encode(r), encode(g), encode(b)]
    }

    /// CIE L* of a film of this fluid, `0` black to `100` white.
    pub fn lightness(&self, film: &Film) -> f32 {
        let y = xyz(&self.reflectance(film))[1];
        let f = if y > 0.008_856 {
            m::powf(y, 1.0 / 3.0)
        } else {
            7.787 * y + 16.0 / 116.0
        };
        116.0 * f - 16.0
    }

    /// **A number that is the same for the same fluid** and, with overwhelming likelihood, different for another: what a cache
    /// keyed on a fluid compares. FNV-1a over the variant and, for a pigment, the bit pattern of every field, so a fluid that is
    /// edited at run time is a different key and a table built for the old one is dropped.
    pub fn key(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |v: u32| {
            for b in v.to_le_bytes() {
                h ^= b as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        };
        eat(match self {
            Fluid::Blood => 0,
            Fluid::Ichor => 1,
            Fluid::Green => 2,
            Fluid::Blue => 3,
            Fluid::Milk => 4,
            Fluid::Custom(_) => 5,
        });
        if let Fluid::Custom(p) = self {
            for v in [
                p.floor,
                p.continuum,
                p.exponent,
                p.scatter,
                p.dries,
                p.dried[0],
                p.dried[1],
                p.dried[2],
            ] {
                eat(v.to_bits());
            }
            for band in &p.bands {
                for v in [band.nm, band.width_nm, band.mua] {
                    eat(v.to_bits());
                }
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bloodstain::spectral::{SO2_ARTERIAL, SO2_VENOUS};

    const PIGMENTS: [Fluid; 4] = [Fluid::Ichor, Fluid::Green, Fluid::Blue, Fluid::Milk];

    fn film(thickness_mm: f32, substrate: f32) -> Film {
        Film {
            thickness_mm,
            so2: SO2_VENOUS,
            substrate,
        }
    }

    /// **Blood is the function it always was, to the bit**: the same reflectance, the same linear and encoded colour, for the films the rest of the
    /// crate asks it about. Every golden of the spectral model stands on this.
    #[test]
    fn blood_is_the_spectral_model_to_the_bit() {
        let bits = |c: [f32; 3]| c.map(f32::to_bits);
        for (thickness_mm, so2, substrate) in [
            (0.0, SO2_VENOUS, 0.5),
            (0.05, SO2_ARTERIAL, 0.5),
            (1.0, SO2_VENOUS, 0.5),
            (3.0, 0.5, 0.9),
            (0.4, 0.2, 0.0),
        ] {
            let f = Film {
                thickness_mm,
                so2,
                substrate,
            };
            assert_eq!(Fluid::Blood.reflectance(&f), spectral::reflectance(&f));
            assert_eq!(
                bits(Fluid::Blood.linear_srgb(&f)),
                bits(spectral::linear_srgb(&f))
            );
            assert_eq!(bits(Fluid::Blood.srgb(&f)), bits(spectral::srgb(&f)));
            assert_eq!(
                Fluid::Blood.lightness(&f).to_bits(),
                spectral::lightness(&f).to_bits()
            );
        }
        assert_eq!(Fluid::default(), Fluid::Blood, "the default fluid is blood");
        assert!(Fluid::Blood.pigment().is_none());
    }

    /// **A thick film is the fluid's hue**, over a substrate it cannot see: green is green, blue is blue, ichor is dark and milk is a pale white with
    /// almost no chroma. The hues are the pigments' (`docs/research/fluids.md`), checked on the colour a renderer reads.
    #[test]
    fn a_thick_film_is_the_fluids_hue() {
        let green = Fluid::Green.srgb(&film(2.0, 0.5));
        assert!(
            green[1] > green[0] + 0.2 && green[1] > green[2] + 0.4,
            "green blood is not green: {green:?}"
        );
        let blue = Fluid::Blue.srgb(&film(2.0, 0.5));
        assert!(
            blue[2] > blue[0] + 0.4 && blue[2] > blue[1] + 0.2,
            "blue blood is not blue: {blue:?}"
        );
        assert!(
            Fluid::Ichor.lightness(&film(2.0, 0.5)) < 20.0,
            "black ichor is not dark: L* {}",
            Fluid::Ichor.lightness(&film(2.0, 0.5))
        );
        let milk = Fluid::Milk.srgb(&film(2.0, 0.5));
        let chroma = milk[0].max(milk[1]).max(milk[2]) - milk[0].min(milk[1]).min(milk[2]);
        assert!(
            Fluid::Milk.lightness(&film(2.0, 0.5)) > 85.0 && chroma < 0.1,
            "milk is not white: {milk:?}"
        );
    }

    /// **Thin melanin is brown and thick melanin is black**: melanin's power law lets the red through a thin film that the blue cannot cross. Over a
    /// white substrate a thin ichor film is lighter than a thick one and redder than it is blue.
    #[test]
    fn thin_ichor_is_brown_and_thick_ichor_is_black() {
        let (thin, thick) = (
            Fluid::Ichor.srgb(&film(0.03, 0.9)),
            Fluid::Ichor.srgb(&film(2.0, 0.9)),
        );
        assert!(
            thin[0] > thin[2] + 0.15,
            "a thin ichor film is not brown: {thin:?}"
        );
        assert!(
            Fluid::Ichor.lightness(&film(0.03, 0.9))
                > Fluid::Ichor.lightness(&film(2.0, 0.9)) + 20.0,
            "{thin:?} {thick:?}"
        );
    }

    /// **An absorber darkens what it lies on, a scatterer whitens it**: over white, `L*` never rises with thickness for ichor, green or blue; over a
    /// dark substrate it never falls for milk. The two directions are the Kubelka-Munk layer's, not an authored ramp.
    #[test]
    fn thicker_darkens_an_absorber_and_whitens_a_scatterer() {
        for fluid in [Fluid::Ichor, Fluid::Green, Fluid::Blue] {
            let mut last = f32::INFINITY;
            for i in 0..=40u32 {
                let l = fluid.lightness(&film(i as f32 * 0.02, 0.9));
                assert!(
                    l <= last + 0.01,
                    "{fluid:?} lightened to L* {l} from {last} at {} mm",
                    i as f32 * 0.02
                );
                last = l;
            }
        }
        let mut last = -1.0;
        for i in 0..=40u32 {
            let l = Fluid::Milk.lightness(&film(i as f32 * 0.02, 0.1));
            assert!(
                l >= last - 0.01,
                "milk darkened to L* {l} from {last} at {} mm",
                i as f32 * 0.02
            );
            last = l;
        }
    }

    /// **A pigment ignores oxygen**: saturation is haemoglobin's, and a fluid that has none gives one colour at both.
    #[test]
    fn a_pigment_has_no_oxygen_saturation() {
        for fluid in PIGMENTS {
            let a = fluid.srgb(&Film {
                thickness_mm: 0.3,
                so2: SO2_ARTERIAL,
                substrate: 0.5,
            });
            let v = fluid.srgb(&Film {
                thickness_mm: 0.3,
                so2: SO2_VENOUS,
                substrate: 0.5,
            });
            assert_eq!(a, v, "{fluid:?} changed with saturation");
        }
    }

    /// **The key tells fluids apart**, and a custom pigment edited in one field is another key: what a table cached on a fluid compares.
    #[test]
    fn the_key_is_the_fluid() {
        let named = [
            Fluid::Blood,
            Fluid::Ichor,
            Fluid::Green,
            Fluid::Blue,
            Fluid::Milk,
        ];
        for (i, a) in named.iter().enumerate() {
            for b in &named[i + 1..] {
                assert_ne!(a.key(), b.key(), "{a:?} and {b:?} share a key");
            }
            assert_eq!(a.key(), a.clone().key());
        }
        let Some(green) = Fluid::Green.pigment() else {
            panic!("green is a pigment")
        };
        let custom = Fluid::Custom(green);
        let edited = Fluid::Custom(Pigment {
            scatter: green.scatter + 0.5,
            ..green
        });
        assert_ne!(
            custom.key(),
            edited.key(),
            "editing a pigment's scattering kept its key"
        );
        assert_ne!(
            custom.key(),
            Fluid::Green.key(),
            "a custom copy of green is the named one's key"
        );
        assert_eq!(
            custom.srgb(&film(1.0, 0.5)),
            Fluid::Green.srgb(&film(1.0, 0.5)),
            "a custom copy of green is another colour"
        );
    }

    /// **Nonsense in is a colour out, never a NaN or a panic**: a pigment with a NaN width, a negative strength or no scattering at all is drawn as what
    /// its finite part says.
    #[test]
    fn a_bad_pigment_is_finite() {
        let bad = Pigment {
            floor: f32::NAN,
            continuum: f32::INFINITY,
            exponent: f32::NAN,
            bands: [
                Band {
                    nm: f32::NAN,
                    width_nm: 0.0,
                    mua: 5.0,
                },
                Band {
                    nm: 550.0,
                    width_nm: -3.0,
                    mua: -2.0,
                },
                Band {
                    nm: 600.0,
                    width_nm: f32::INFINITY,
                    mua: 1.0,
                },
            ],
            scatter: -1.0,
            dried: [f32::NAN, -1.0, 9.0],
            dries: f32::NAN,
        };
        for d in [0.0, 0.05, 1.0, f32::NAN, -2.0] {
            let c = Fluid::Custom(bad).srgb(&film(d, 0.5));
            assert!(
                c.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
                "a bad pigment at {d} mm drew {c:?}"
            );
        }
    }

    /// **A fluid is authored as data**: the named ones by name, a custom one by its numbers; a field nobody knows is refused, as every settings block
    /// in this crate refuses it.
    #[cfg(feature = "serde")]
    #[test]
    fn a_fluid_round_trips_through_ron() {
        for fluid in [
            Fluid::Blood,
            Fluid::Green,
            Fluid::Milk,
            Fluid::Custom(Pigment {
                dries: 0.25,
                ..Fluid::Blue.pigment().expect("blue is a pigment")
            }),
        ] {
            let text = ron::to_string(&fluid).expect("a fluid serialises");
            let back: Fluid = ron::from_str(&text).expect("what a fluid wrote it reads");
            assert_eq!(back, fluid, "{text}");
        }
        assert_eq!(
            ron::from_str::<Fluid>("Green").ok(),
            Some(Fluid::Green),
            "a named fluid is written as its name"
        );
        let typo = "Custom((floor: 0.0, continuum: 0.0, exponent: 3.0, bands: ((nm: 550.0, width_nm: 10.0, mua: 1.0), (nm: 550.0, width_nm: 10.0, mua: 0.0), (nm: 550.0, width_nm: 10.0, mua: 0.0)), scatter: 1.0, dried: (1.0, 1.0, 1.0), dries: 1.0, colour: 3))";
        assert!(
            ron::from_str::<Fluid>(typo).is_err(),
            "a pigment with a field nobody knows was accepted"
        );
    }
}
