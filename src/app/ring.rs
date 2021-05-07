use std::collections::HashSet;

use crate::app::{downloader::make_download, ring::permutation::{build_permutations, Permutation, Transforms}};

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
    allowed_xforms: AllowedTransformFamiles,
    perm_count: u64,
    permutations: Vec<Permutation>,
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            n_colours: 3,
            n_beads: 5,
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
            allowed_xforms,
            perm_count,
            ..
        } = self;
        let mut changed = *perm_count == 0;
        let max_beads = (64. / (*n_colours as f64).log2()) as u64;
        let max_colours = (64. / (*n_beads as f64)).exp2() as u64;
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
            perm_count,
            permutations,
            ..
        } = self;

        let mut orbits = 0;
        let mut fixed = 0;

        // Special-case no rotation;
        orbits += 1;
        fixed += n_colours.pow(*n_beads as u32);

        if matches!(*allowed_xforms, Rotate | RotateAndFlip) {
            for i in 1..*n_beads {
                orbits += 1;
                fixed += n_colours.pow(gcd(*n_beads, i) as u32);
            }
        }

        if matches!(*allowed_xforms, RotateAndFlip) {
            if *n_beads % 2 == 0 {
                // We can flip on edges, or corners
                // Edges will fix n_beads/2, and we have n_beads/2 flip axes
                // While corners will fix n_beads/2 + 1, and again we have n_beads/2 flip axes
                orbits += *n_beads;
                fixed += (*n_beads / 2) * n_colours.pow((*n_beads / 2) as u32);
                fixed += (*n_beads / 2) * n_colours.pow((*n_beads / 2 + 1) as u32);
            } else {
                // Each corner flip is an edge flip and vice versa.
                // Each will fix (n - 1) / 2 + 1
                orbits += *n_beads;
                // Rely on rounding down so we don't need to subtract 1 first
                fixed += *n_beads * n_colours.pow((*n_beads / 2 + 1) as u32);
            }
        }

        *perm_count = fixed / orbits;

        let transforms = Transforms::new(*n_beads, *allowed_xforms);
        if *perm_count <= 10000 {
            let mut seen = HashSet::with_capacity(*perm_count as usize);
            build_permutations(
                &transforms,
                *n_beads,
                *n_colours,
                0,
                Permutation::new(),
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
                let message = format!("Error: Expected to find {} results, but found {} instead. Please report this as a bug to mdsherry@gmail.com", *perm_count, permutations.len());
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
        let Self { n_beads, n_colours, allowed_xforms, permutations, ..} = self;
        let mut rv = vec![];
        for permutation in permutations {
            let mut entry = vec![];
            for y in 0..*n_beads {
                entry.push(permutation.get(y));
            }
            rv.push(entry)
        }
        let name = format!("{} beads {} col {:?}.json", n_beads, n_colours, allowed_xforms);
        let bytes =serde_json::to_vec(&rv).unwrap();
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
