use std::collections::HashSet;

use crate::app::{
    bag_draw::*,
    downloader::make_download,
    ring::permutation::{build_permutations, Permutation, Transforms},
};

use super::palettes::Palette;
use eframe::egui;

mod permutation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowedTransformFamiles {
    NoTransforms,
    Rotate,
    RotateAndFlip,
}
use AllowedTransformFamiles::*;

pub struct Ring {
    n_colours: u64,
    n_beads: u64,
    limit_repeats: bool,
    max_repeats: u64,
    allowed_xforms: AllowedTransformFamiles,
    perm_count: u64,
    permutations: Vec<Permutation>,
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            n_colours: 3,
            n_beads: 5,
            limit_repeats: false,
            max_repeats: 0,
            allowed_xforms: Rotate,
            perm_count: 0,
            permutations: vec![],
        }
    }
}
impl Ring {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn settings(&mut self, ui: &mut egui::Ui) -> bool {
        let Self {
            n_beads,
            n_colours,
            limit_repeats,
            max_repeats,
            allowed_xforms,
            perm_count,
            ..
        } = self;
        let mut changed = *perm_count == 0;
        let max_beads = (64. / (*n_colours as f64).log2()) as u64;
        let max_colours = (64. / (*n_beads as f64)).exp2() as u64;
        let min_max_repeats = (*n_beads + *n_colours - 1) / *n_colours;
        let max_max_repeats = *n_beads;
        ui.heading("Settings");

        changed |= ui
            .add(
                egui::Slider::new(n_colours, 2..=max_colours.min(8))
                    .text("Number of colours")
                    .clamp_to_range(true),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(n_beads, 2..=max_beads.min(10))
                    .text("Number of beads")
                    .clamp_to_range(true),
            )
            .changed();
        changed |= ui
            .checkbox(limit_repeats, "Limit colour repetions")
            .changed();
        if *limit_repeats {
            if *max_repeats < min_max_repeats {
                changed = true;
                *max_repeats = min_max_repeats;
            }
            if *max_repeats > max_max_repeats {
                changed = true;
                *max_repeats = max_max_repeats;
            }
            changed |= ui
                .add(
                    egui::Slider::new(max_repeats, min_max_repeats..=max_max_repeats)
                        .text("Maximum repeats")
                        .clamp_to_range(true),
                )
                .changed();
        }
        ui.label("Possible transforms:");
        changed |= ui
            .radio_value(allowed_xforms, NoTransforms, "No transforms")
            .changed();
        changed |= ui
            .radio_value(allowed_xforms, Rotate, "Rotations")
            .changed();
        changed |= ui
            .radio_value(allowed_xforms, RotateAndFlip, "Rotation and flip")
            .changed();

        ui.label(format!("There are {} distinct tiles", perm_count));

        changed
    }

    pub fn recompute_perms(&mut self) {
        let Self {
            allowed_xforms,
            n_beads,
            n_colours,
            limit_repeats,
            max_repeats,
            perm_count,
            permutations,
            ..
        } = self;

        let (orbits, fixed) = if *limit_repeats {
            limited_count(*n_colours, *n_beads, *max_repeats, *allowed_xforms)
        } else {
            unlimited_count(*n_colours, *n_beads, *allowed_xforms)
        };

        if orbits > 0 {
            *perm_count = fixed / orbits;
        } else {
            *perm_count = 0;
        }

        let transforms = Transforms::new(*n_beads, *allowed_xforms);
        if *perm_count <= 10000 {
            let mut seen = HashSet::with_capacity(*perm_count as usize);
            let mut colour_counts = vec![0; *n_colours as usize];
            build_permutations(
                &transforms,
                *n_beads,
                *n_colours,
                0,
                Permutation::new(),
                &mut colour_counts,
                if *limit_repeats {
                    *max_repeats
                } else {
                    u64::MAX
                },
                &mut seen,
            );
            permutations.clear();
            permutations.extend(seen);
            permutations.sort_unstable();
        } else {
            permutations.clear();
        }
    }

    pub fn render_results(&self, palette: &Palette, ui: &mut egui::Ui) {
        let Self {
            n_beads,
            perm_count,
            permutations,
            ..
        } = self;
        if *perm_count > 10000 {
            ui.label("Too many (> 10,000) variants to display");
        } else {
            if *perm_count as usize != permutations.len() {
                let mut message = format!("Error: Expected to find {} results, but found {} instead. Please report this as a bug to mdsherry@gmail.com", *perm_count, permutations.len());

                if self.limit_repeats {
                    for i in 1..self.n_beads {
                        let rotations = gcd(self.n_beads, i);
                        let orbit_size = *n_beads / rotations;
                        // We basically have a bag of max_repeats / orbit_size stones of each colour, from which we will be making `rotations` draws.
                        // How many distinct outcomes are there?
                        let n_combis =
                            simple_count(rotations, self.n_colours, self.max_repeats / orbit_size);
                        let expected = n_combis / self.n_beads;
                        let mut count = 0;
                        for perm in permutations {
                            let mut okay = true;
                            for j in 0..self.n_beads {
                                let next = (j + rotations) % self.n_beads;
                                if perm.get(j) != perm.get(next) {
                                    okay = false;
                                    break;
                                }
                            }
                            if okay {
                                count += 1;
                            }
                        }
                        if expected != count {
                            message.push_str(&format!("\nExpected {expected} shapes with {rotations}-wise ({i} step) rotational symmetry, but saw {count}"));
                        }
                    }
                }
                ui.colored_label(egui::Color32::from_rgb(255, 0, 0), message);
            }
            if cfg!(target_arch = "wasm32") && ui.button("Download JSON").clicked() {
                let (name, bytes) = self.export_json();
                make_download(&name, &bytes, "application/json");
            }
            ui.horizontal_wrapped(|ui| {
                for permutation in permutations {
                    let (rect, _response) =
                        ui.allocate_exact_size((60., 60.).into(), egui::Sense::hover());
                    let delta = std::f64::consts::TAU / (*n_beads as f64);
                    let distance = 15.;
                    let radius = {
                        let x1 = distance;
                        let y1 = 0.;
                        let x2 = delta.cos() * distance;
                        let y2 = delta.sin() * distance;
                        ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt() / 2.
                    } as f32;

                    let mut theta: f64 = if *n_beads % 2 == 0 { delta / 2. } else { 0. };
                    for n in 0..*n_beads {
                        let x = theta.sin() * distance;
                        let y = -theta.cos() * distance;
                        ui.painter().circle_filled(
                            rect.center() + egui::vec2(x as f32, y as f32),
                            radius,
                            palette[permutation.get(n) as usize],
                        );
                        theta += delta;
                    }
                }
            });
        }
    }
    pub fn export_json(&self) -> (String, Vec<u8>) {
        let Self {
            n_beads,
            n_colours,
            allowed_xforms,
            permutations,
            ..
        } = self;
        let mut rv = vec![];
        for permutation in permutations {
            let mut entry = vec![];
            for y in 0..*n_beads {
                entry.push(permutation.get(y));
            }
            rv.push(entry)
        }
        let name = format!(
            "{} beads {} col {:?}.json",
            n_beads, n_colours, allowed_xforms
        );
        let bytes = serde_json::to_vec(&rv).unwrap();
        (name, bytes)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    // a and b should be small enough that trial division is feasible
    // Heck, we could even just have a look-up table
    assert!(b <= a);
    for i in (1..=b).rev() {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }
    1
}

fn limited_count(
    n_colours: u64,
    n_beads: u64,
    max_repeats: u64,
    allowed_xforms: AllowedTransformFamiles,
) -> (u64, u64) {
    // Special-case no rotation;
    let (mut orbits, mut fixed) = limited_base_count(n_colours, n_beads, max_repeats);
    if matches!(allowed_xforms, Rotate | RotateAndFlip) {
        let (extra_orbits, extra_fixed) = limited_rotate_count(n_colours, n_beads, max_repeats);
        orbits += extra_orbits;
        fixed += extra_fixed;
    }
    if matches!(allowed_xforms, RotateAndFlip) {
        let (extra_orbits, extra_fixed) = limited_flip_count(n_colours, n_beads, max_repeats);
        orbits += extra_orbits;
        fixed += extra_fixed;
    }
    (orbits, fixed)
}

fn limited_base_count(n_colours: u64, n_beads: u64, max_repeats: u64) -> (u64, u64) {
    let n_combis = simple_count(n_beads, n_colours, max_repeats);
    if n_combis > 0 {
        (1, n_combis)
    } else {
        (0, 0)
    }
}

fn limited_rotate_count(n_colours: u64, n_beads: u64, max_repeats: u64) -> (u64, u64) {
    let mut orbits = 0;
    let mut fixed = 0;

    for i in 1..n_beads {
        // This is the number of 'free' points. If the GCD == 1, then we require that all points the be same colour. If it's 2, then we have two points that can be independently coloured, and so on.
        // E.g. if i == 2, and n_beads == 6, then bead 0 maps to bead 2 maps to bead 4, while bead 1 maps to bead 3 maps to bead 5. If i = 4, then bead 0 maps to bead 4 maps to bead 2, etc.
        let rotations = gcd(n_beads, i);

        let orbit_size = n_beads / rotations;
        // We basically have a bag of max_repeats / orbit_size stones of each colour, from which we will be making `rotations` draws.
        // How many distinct outcomes are there?
        let n_combis = simple_count(rotations, n_colours, max_repeats / orbit_size);
        fixed += n_combis;
        orbits += 1;
    }

    (orbits, fixed)
}

fn limited_flip_count(n_colours: u64, n_beads: u64, max_repeats: u64) -> (u64, u64) {
    let mut orbits = 0;
    let mut fixed = 0;
    let reduced_max = max_repeats / 2;
    if n_beads % 2 == 0 {
        // We can flip on edges, or corners
        // Edges will fix n_beads/2, and we have n_beads/2 flip axes
        // While corners will fix n_beads/2 + 1, and again we have n_beads/2 flip axes
        orbits += n_beads;

        // For edge flips, it's straight-forward
        let n_combis = (n_beads / 2) * simple_count(n_beads / 2, n_colours, reduced_max);
        fixed += n_combis;
        // For corner flips, we don't have the same matching constraints.
        // If both axes of the corner match in colour, that one colour will be less available for the other points
        // Otherwise, it will depend on whether max_repeats is even or odd.
        let mut counts = vec![0; max_repeats as usize + 1];
        counts[max_repeats as usize] = n_colours;
        let mut draws = vec![2; (n_beads - 2) as usize / 2];
        draws.push(1);
        draws.push(1);
        let n_combis = (n_beads / 2) * count3(&draws, &mut counts);
        fixed += n_combis;
    } else {
        // Each corner flip is an edge flip and vice versa.
        // Each will fix (n - 1) / 2 + 1
        orbits += n_beads;
        let mut counts = vec![0; max_repeats as usize / 2 + 1];
        counts[reduced_max as usize] = n_colours - 1;
        counts[(max_repeats - 1) as usize / 2] += 1;
        let n_combis = n_beads * n_colours * count2((n_beads - 1) / 2, &mut counts);
        // fixed += n_beads * n_colours.pow((n_beads / 2 + 1) as u32);
        fixed += n_combis;
    }
    (orbits, fixed)
}

fn unlimited_count(
    n_colours: u64,
    n_beads: u64,
    allowed_xforms: AllowedTransformFamiles,
) -> (u64, u64) {
    let (mut orbits, mut fixed) = unlimited_base_count(n_colours, n_beads);
    if matches!(allowed_xforms, Rotate | RotateAndFlip) {
        let (extra_orbits, extra_fixed) = unlimited_rotate_count(n_colours, n_beads);
        orbits += extra_orbits;
        fixed += extra_fixed;
    }
    if matches!(allowed_xforms, RotateAndFlip) {
        let (extra_orbits, extra_fixed) = unlimited_flip_count(n_colours, n_beads);
        orbits += extra_orbits;
        fixed += extra_fixed;
    }
    (orbits, fixed)
}

fn unlimited_base_count(n_colours: u64, n_beads: u64) -> (u64, u64) {
    (1, n_colours.pow(n_beads as u32))
}

fn unlimited_rotate_count(n_colours: u64, n_beads: u64) -> (u64, u64) {
    let mut orbits = 0;
    let mut fixed = 0;

    for i in 1..n_beads {
        let rotations = gcd(n_beads, i);
        orbits += 1;
        fixed += n_colours.pow(rotations as u32);
    }

    (orbits, fixed)
}

fn unlimited_flip_count(n_colours: u64, n_beads: u64) -> (u64, u64) {
    let mut orbits = 0;
    let mut fixed = 0;
    if n_beads % 2 == 0 {
        // We can flip on edges, or corners
        // Edges will fix n_beads/2, and we have n_beads/2 flip axes
        // While corners will fix n_beads/2 + 1, and again we have n_beads/2 flip axes
        orbits += n_beads;
        fixed += (n_beads / 2) * n_colours.pow((n_beads / 2) as u32);
        fixed += (n_beads / 2) * n_colours.pow((n_beads / 2 + 1) as u32);
    } else {
        // Each corner flip is an edge flip and vice versa.
        // Each will fix (n - 1) / 2 + 1
        orbits += n_beads;
        // Rely on rounding down so we don't need to subtract 1 first
        fixed += n_beads * n_colours.pow((n_beads / 2 + 1) as u32);
    }
    (orbits, fixed)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::{
        limited_count,
        permutation::{build_permutations, Permutation, Transforms},
        unlimited_count, AllowedTransformFamiles,
    };

    // #[test]
    // fn test_ring_limited2() {
    //     let xform = AllowedTransformFamiles::RotateAndFlip;
    //     let beads = 3;
    //     let transforms = Transforms::new(beads, xform);
    //     let colours = 2;
    //     let max_repeats = 3;
    //     let (orbits, fixed) = limited_count(colours, beads, max_repeats, xform);
    //     assert_eq!(0, fixed % orbits, "Fixed count indivisible by orbits on {colours} colours, {beads} beads, {max_repeats} max repeats and {xform:?} transform");
    //     let perm_count = if orbits == 0 {
    //         0
    //     } else {
    //         fixed / orbits
    //     };
    //     if perm_count <= 10000 {
    //         let mut seen = HashSet::with_capacity(perm_count as usize);
    //         let mut colour_counts = vec![0; colours as usize];
    //         build_permutations(
    //             &transforms,
    //             beads,
    //             colours,
    //             0,
    //             Permutation::new(),
    //             &mut colour_counts,
    //             max_repeats,
    //             &mut seen,
    //         );
    //         assert_eq!(seen.len(), perm_count as usize, "Mismatch on {colours} colours, {beads} beads, {max_repeats} max repeats and {xform:?} transform");
    //     }
    // }

    #[test]
    fn test_ring_limited() {
        for xform in [
            AllowedTransformFamiles::NoTransforms,
            AllowedTransformFamiles::Rotate,
            AllowedTransformFamiles::RotateAndFlip,
        ] {
            for beads in 2..10 {
                let transforms = Transforms::new(beads, xform);
                for colours in 1..8 {
                    let min_max_repeats = (beads + colours - 1) / colours;
                    let max_max_repeats = beads;
                    for max_repeats in min_max_repeats..=max_max_repeats {
                        let (orbits, fixed) = limited_count(colours, beads, max_repeats, xform);
                        assert_eq!(0, fixed % orbits, "Fixed count indivisible by orbits on {colours} colours, {beads} beads, {max_repeats} max repeats and {xform:?} transform");
                        let perm_count = if orbits == 0 { 0 } else { fixed / orbits };
                        if perm_count <= 10000 {
                            let mut seen = HashSet::with_capacity(perm_count as usize);
                            let mut colour_counts = vec![0; colours as usize];
                            build_permutations(
                                &transforms,
                                beads,
                                colours,
                                0,
                                Permutation::new(),
                                &mut colour_counts,
                                max_repeats,
                                &mut seen,
                            );
                            assert_eq!(seen.len(), perm_count as usize, "Mismatch on {colours} colours, {beads} beads, {max_repeats} max repeats and {xform:?} transform");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_ring_unlimited() {
        for xform in [
            AllowedTransformFamiles::NoTransforms,
            AllowedTransformFamiles::Rotate,
            AllowedTransformFamiles::RotateAndFlip,
        ] {
            for beads in 2..10 {
                let transforms = Transforms::new(beads, xform);
                for colours in 1..8 {
                    let (orbits, fixed) = unlimited_count(colours, beads, xform);
                    let perm_count = if orbits == 0 { 0 } else { fixed / orbits };
                    if perm_count <= 10000 {
                        let mut seen = HashSet::with_capacity(perm_count as usize);
                        let mut colour_counts = vec![0; colours as usize];
                        build_permutations(
                            &transforms,
                            beads,
                            colours,
                            0,
                            Permutation::new(),
                            &mut colour_counts,
                            u64::MAX,
                            &mut seen,
                        );
                        assert_eq!(
                            seen.len(),
                            perm_count as usize,
                            "Mismatch on {colours} colours, {beads} beads and {xform:?} transform"
                        );
                    }
                }
            }
        }
    }
}
