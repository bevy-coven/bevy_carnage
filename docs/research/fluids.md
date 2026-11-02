# Fluids: blood that is not red (iteration 20)

The crate's blood is physics all the way down: `bloodstain::spectral` turns a film's thickness, its oxygen saturation and what it lies on into a colour (Kubelka-Munk over Bosschaart et al.'s measured whole-blood optics), `dry` ages that colour, and every surface the preset dresses (a canvas texel, a cut face, a cloth, a strand of gut) takes its blood from that one model. That is the right model for red, and it is the only colour the crate can draw. A game that needs the blood **not** to be red has nowhere to put the request: `GoreTier::Stylised` draws nothing at all in the preset (its substitute palette is a particle colour, and the particles are `vfx`, which cannot build on Bevy 0.20), and a cartoon, a robot, an alien or a rating-board build must either lose the blood or fork the crate.

The item: a `Fluid` dial. It replaces the **optics and the ageing** of the blood a preset draws and nothing else, so every digest and every golden of the simulation stays what it is, and a surface wears the fluid the way it wears blood: thin and tinted over a pale substrate, deep and opaque in a pool, a crust when it dries.

## Sources

**Physics** (what a fluid's colour is):

- **The film model is already the crate's**: Kubelka and Munk's two-flux layer, `K = 2 mu_a`, `S = 3/4 mu_s'`, over a substrate of reflectance `Rg`, then CIE 1931 under D65 (`src/bloodstain/spectral.rs`, quoted there from Bosschaart, Edelman, Aalders, van Leeuwen and Faber, Lasers Med Sci 29 (2014), `doi:10.1007/s10103-013-1446-7`). A fluid is **a different `mu_a(lambda)` and `mu_s'` in the same model**; nothing about the layer solution is blood's.
- **Black**: melanin. Jacques, "Melanosome Absorption Coefficient", OMLC (`https://omlc.org/spectra/melanin/mua.html`): "the general shape of the melanosome absorption spectrum is approximated: `mu_a = 1.70x10^12 nm^-3.48 [cm^-1]` for skin", with "ten-fold variation to be expected" in the concentration. A power law that falls steeply with wavelength is why a thin melanin film is **brown** (red passes) and a thick one black. The review it summarises is Jacques, "Optical properties of biological tissues: a review", Phys. Med. Biol. 58 (2013) R37, `doi:10.1088/0031-9155/58/11/r37`.
- **Green**: biliverdin, the bile pigment of the **Prasinohaema** skinks, whose blood is green. Austin and Jessing, "Green-blood pigmentation in lizards", Comp. Biochem. Physiol. A 109 (1994) 619-626, `doi:10.1016/0300-9629(94)90201-1`: plasma biliverdin of 714 to 1020 umol/l, "the highest concentration of plasma biliverdin measured for any organism". The colour is two absorption bands and a window between them: a Soret band near 375-385 nm and a Q band near 650-700 nm, so green is transmitted (Sci. Adv. frog-camouflage paper, `https://www.science.org/doi/10.1126/sciadv.aeg5510`: biliverdin "exhibits green coloration due to absorption in the violet (Soret band) and red (Q band) regions"; a patent's spectra put the peaks at "385 nm and 650 nm-660 nm").
- **Blue**: haemocyanin, the copper blood of octopus and horseshoe crab, colourless deoxygenated and blue oxygenated. Solomon et al., PNAS 76 (1979) 2094, `doi:10.1073/pnas.76.5.2094`: "the bands at 570 and 486 nm are assigned as components of the peroxide pi* to copper charge transfer", with the 345 nm band (extinction 20 000) in the ultraviolet and a 585 and a 625 nm shoulder: "a combination of the 585 nm and 625 nm absorbances explain the blue color of the oxyhemocyanin complex" (Libretexts summary of the same). The octopus one is Tallandini and Salvato, Biochim. Biophys. Acta (1975), on its copper absorption bands.
- **White**: milk, which is what a **white** blood is made of on a film set and what white blood is in optics: all scattering, almost no absorption. Stocker et al., "Broadband Optical Properties of Milk", Appl. Spectrosc. (2016): absorption coefficients "between 1 x 10^-3 and 8 x 10^-3 mm^-1", scattering of the order of the fat globules; Aernouts et al., J. Dairy Sci. 98 (2015) 6727, `doi:10.3168/jds.2015-9630`, the bulk coefficients of raw milk (fat globule size and content decide the scattering); a reduced scattering coefficient of 60 cm^-1 (6 mm^-1) for 3 % fat in the green.

**Horror-film practice** (the reference set below): the non-red fluid is a staple of the genre and of its rating.

- *Evil Dead II* (1987): "because the partners were contractually obliged to deliver an 'R' rated picture, everyone thought it important to emphasize bodily fluids in green, black, yellow, or any other colour except red" (Book of the Dead, `https://www.bookofthedead.ws/website/evil_dead_2_special_fx.html`); the possessed Henrietta spits black and green, and "the blood turns black and sucks back into the wall" (IMDb alternate versions, `https://www.imdb.com/title/tt0092991/alternateversions/`; outlawvern, `https://outlawvern.com/2022/01/12/evil-dead-ii/`).
- *Alien* (1979): Ash, the android, bleeds **white**, and the white was **condensed milk** (Empire, `https://www.empireonline.com/movies/features/classic-scene-alien/`: "white liquid dribbles from his mouth"; SlashFilm, `https://www.slashfilm.com/833734/how-an-experiment-on-the-alien-set-became-one-of-ridley-scotts-weirdest-calling-cards/`, on an improvised drop of milk in the fight; the "does anybody have an eyedropper full of milk?" line and the condensed milk are as the fan wiki `https://ai.miraheze.org/wiki/Ash_(android)` reports them, which I could not check against a primary source). The reading of the effect, in the same sources, is that a milky white substance from what looks like a human body is more disturbing than red.
- *Re-Animator* (1985): the reagent is **green and luminous**, and was "the fluid from inside commercial glow-sticks, mixed with a little bit of highlighter ink" (popcornizer, `https://popcornizer.com/1985/re-animator/`). Its albedo is not this crate's to reproduce (it is an emitter), its **hue** is.

## Craft principle

The practical-effects rule behind every one of these: **the colour carries the meaning, the surface carries the realism.** Raimi's green and black, Scott's milk and Gordon's glow all keep blood's *behaviour* (it sprays, runs, pools, beads, clots) and change only its hue, which is exactly the split this crate has already made between the simulation (a CPU model that hashes) and the look (a table the shader and the canvases read). The quote that licenses the design is the ESRB's own definition of the descriptor a non-red build earns: **"Animated Blood: discolored and/or unrealistic depictions of blood"** (ESRB content descriptors; `https://www.wikidata.org/wiki/Q60316460`). A fluid dial is how a game reaches that descriptor without losing the hit confirmation.

## How shipping games do it

- **German releases for the USK** recoloured: "blood recolored to green or black" and the humans made robots (ResetEra summary, `https://www.resetera.com/threads/games-censoring-and-localization-in-germany.5666/`); *Carmageddon*'s German build "the red blood was changed to black 'oil'" (`https://carmageddon.fandom.com/wiki/Blood_Patch`), *Half-Life 2*'s "blood was turned black/grey" (The Censorship of German Video Games, `https://files.core.ac.uk/download/pdf/36693056.pdf`); the UK *Carmageddon* had zombies with **green** blood. In every case the **sprite or texture was repainted** for the one colour and the whole pipeline downstream was left red, which is the cost this item removes: one dial, every surface.
- **The substitution rule of this crate's own policy** (`src/policy.rs`, from Vermintide 2 and Gears of War 4): "reduction is substitution, never deletion". At `GoreTier::Stylised` the preset paints nothing. A game that wants the substitute to be a **fluid** (green, black, white) rather than nothing has no tier for it; `Fluid` is the lever, and the tier decision stays the game's.
- The film model keeps WebGL2's budget: a fluid changes the **contents** of the blood table the flesh material already binds, never the shader or a texture slot.

## Model adopted

`Fluid` (`bloodstain::spectral`): `Blood` (the Bosschaart table, the oxygen-saturation dial, the three-stop haemoglobin age walk: **exactly what the crate has done since 0.2**), four named pigments (`Ichor`, `Green`, `Blue`, `Milk`) and `Custom(Pigment)`.

A `Pigment` is a turbid liquid with an absorption spectrum and a flat reduced scattering:

```text
mu_a(lambda) = floor + continuum * (lambda / 500 nm)^-exponent
             + sum over bands of  peak_b * exp(-1/2 ((lambda - centre_b) / width_b)^2)
K = 2 mu_a        S = 3/4 mu_s'
```

and a film of it is `kubelka_munk(K, S, d, Rg)` per wavelength, then the same CIE integration. **Ageing is its own, for want of any measured walk for these fluids** (and it says so): a pigment keeps its hue and loses its freshness by a per-channel multiplier, `fresh * lerp(1, dried, t * dries)`, so a thin film stays lighter than a thick one as blood's walk guarantees, nothing crosses, and `dries` sets how far it goes (`0` is a fluid that never sets, black oil, `1` blood's pace). Its roughness rises with `rim * dries`; there is **no serum halo** (that is blood's red cells separating from plasma).

The dial lives where the colour is made: **`WetSettings::fluid`** (the canvases' resource, next to `so2` and `film_depth_mm`). The flesh tables bake from it at start-up (`FleshTables`), the strands' soak table builds from it, and the preset reads it from the one resource. **One authority**: a second copy in `BloodSettings` (spatter physics) would be the drift `WetSettings::blood()` already warns about.

## Parameters

Filled in with the tuned values at the end of the iteration (every constant says "this crate's own", with the reference it was matched to).

## Validation targets

1. **Blood is untouched.** `Fluid::Blood` reproduces `spectral::srgb`, `appearance_with_fresh`, `blood_lut`, `aged_blood_lut`, `luts` and `soak_tint` to the bit; every frozen golden (`the_spectral_model_is_frozen`, `the_flesh_tables_are_frozen`, `the_aged_blood_table_is_frozen`, the canvases' digests) passes unchanged; every recorder digest equals 0.8.13's.
2. **A fluid moves colour and nothing else.** The same scripted run under each fluid prints the same `digest` as under blood (`capture_skinned`, `capture_preset`), and a canvas stamped under each fluid has one `digest()` and different pixels.
3. **The hue is the hue.** A thick film is green-dominant for `Green` (G above R and B by 20 code values or more), blue-dominant for `Blue`, `L*` under 20 for `Ichor` and over 85 with chroma under 15 for `Milk`; a thin `Ichor` film over white is **browner** than a thick one (R above B), the melanin signature.
4. **Thicker is a stronger colour, in the right direction**: over a mid-grey substrate `L*` falls strictly with thickness for an absorbing fluid, and a milk film over a dark substrate **rises** (a scatterer whitens what is under it).
5. **Ageing only dulls.** No channel of an aged pigment film rises, `Ichor` stays glossier than blood at the end of its drying and `Milk` dries to a crust.
6. **Every surface wears it**: a hit under `Green` gives the target's canvas green texels, a cut face's blood film (the LUT) is green, a cloth's soak is green, a spilled strand's soak tint is green-dominant: none is red.
7. **Cost**: `bench_carnage` is unchanged within noise (no CPU path moves under `Blood`); a fluid's CPU cost is the same sixteen-level table a canvas builds today, once per settings change.
8. **The look**: a three-or-more fluid sheet of one scripted scene (a chest hit, a slash, the floor pools) beside the reference stills below, each judged on hue, body and age.

## What great looks like

The same scene, in blood and in each fluid, and **nobody asks which one is the real one**: the green is a wet lime-green slick that beads on a wall and pools with a darker, olive middle and a dull, tacky rim as it sets (*Re-Animator*'s reagent, *Evil Dead II*'s goo); the black is an oil: glossy, near-black in a pool, brown where it is thin on a pale shirt, and it stays glossy long after red would have crusted (Carmageddon's oil; *Evil Dead II*'s black); the white is milk: translucent and bluish where it is a thin film over skin, an opaque cream in a pool, with a yellowing skin when it dries (*Alien*'s Ash); the blue is a clear, saturated cyan-blue (an octopus or a horseshoe crab). The simulation is the same: the same spray, runs, craters and pieces.

## Reference set

Kept under `/tmp/c20/refs/` only, never committed; URLs only here.

1. **Re-Animator (1985), the reagent** (`https://musingsofamiddleagedgeek.blog/2020/10/19/re-animator-1985-is-still-kicking-and-screaming-after-35-years/`, the first still): a vial of **luminous lime green**; sampled core `(151, 231, 21)` sRGB, the rim `(90, 161, 21)`. It glows: its albedo is darker and the hue is the claim (yellow-green, no blue). Licence: copyrighted still, cited by URL.
2. **Alien (1979), Ash's milk** (`https://www.slashfilm.com/833734/how-an-experiment-on-the-alien-set-became-one-of-ridley-scotts-weirdest-calling-cards/`, "the white android blood wasn't planned" and "it's more uncomfortable than seeing red blood"): a **white translucent film** on a face: thick it is an opaque cool white `(181, 163, 157)` under the set's warm light, thin it lets the skin through as a pink-white. Copyrighted, cited.
3. **Evil Dead II (1987), the R-rating fluids** (text: Book of the Dead and IMDb above; no clean still of the black or green fluid was found, so the colour targets for them are the texts' and the pigments' physics): black, "turns black and sucks back into the wall", green goo from Henrietta.
4. **Carmageddon's German build, black "oil"** (`https://carmageddon.fandom.com/wiki/Blood_Patch`): the shipping-game precedent for a black fluid over the same game.

## Deviations

- A pigment's ageing walk is a design default (a per-channel multiplier over the fresh colour), not a measured one: there is no paper for how glow-stick fluid, condensed milk or an invented ichor dries. It is monotone and bounded, and says so in its doc comment.
- The bruise (`DermisCanvas`) is human skin physiology (haemoglobin and bilirubin in a dermis) and stays what it is under a fluid; so does the tissue under a cut face (`cross_section`'s strip). A game with green blood does not use the blunt-bruise dial, or accepts a human under-skin.
- One fluid per app (the flesh table is one image): a scene with two species of blood is the follow-up (a per-body fluid), recorded in the log.
- The decals and particles of `vfx` (unbuildable on Bevy 0.20) keep their own constants.

## Pending index

Not in the corpus and not downloadable here (paywalled), cited by what the open pages quote: Jacques, Phys. Med. Biol. 58 (2013) R37 (`doi:10.1088/0031-9155/58/11/r37`); Austin and Jessing 1994 (`doi:10.1016/0300-9629(94)90201-1`); Solomon et al., PNAS 76 (1979) 2094 (`doi:10.1073/pnas.76.5.2094`); Tallandini and Salvato 1975; Aernouts et al. 2015 (`doi:10.3168/jds.2015-9630`); Stocker et al. 2016.
