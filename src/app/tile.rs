use std::collections::HashSet;

use crate::app::downloader::make_download;

use super::palettes::Palette;
use eframe::egui;

mod permutation;
mod transforms;
use permutation::{build_permutations, Permutation, Transforms};
use transforms::*;
use AllowedTransformFamiles::*;

pub struct Tile {
    n_colours: u64,
    width: u64,
    height: u64,
    allowed_xforms: AllowedTransformFamiles,
    perm_count: u64,
    permutations: Vec<Permutation>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            n_colours: 3,
            width: 2,
            height: 2,
            allowed_xforms: Rotate,
            perm_count: 0,
            permutations: vec![],
        }
    }
}
impl Tile {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn settings(&mut self, ui: &mut egui::Ui) -> bool {
        let Self {
            height,
            width,
            n_colours,
            allowed_xforms,
            perm_count,
            ..
        } = self;
        let mut changed = *perm_count == 0;
        let max_dimension_product = 64. / (*n_colours as f64).log2();
        let max_width = (max_dimension_product / *height as f64) as u64;
        let max_height = (max_dimension_product / *width as f64) as u64;
        let max_colours = (64. / (*width * *height) as f64).exp2() as u64;
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
                egui::Slider::new(width, 1..=max_width.min(8))
                    .text("Width")
                    .clamp_to_range(true),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(height, 1..=max_height.min(8))
                    .text("Height")
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
            width,
            height,
            n_colours,
            perm_count,
            permutations,
            ..
        } = self;
        let square = *width == *height;
        let mut orbits = 0;
        let mut fixed = 0;
        if NoXform::applicable(*allowed_xforms, square) {
            orbits += NoXform::ORBITS;
            fixed += NoXform::total_fixed(*width, *height, *n_colours);
        }
        if Rot90::applicable(*allowed_xforms, square) {
            orbits += Rot90::ORBITS;
            fixed += Rot90::total_fixed(*width, *height, *n_colours);
        }
        if Rot180::applicable(*allowed_xforms, square) {
            orbits += Rot180::ORBITS;
            fixed += Rot180::total_fixed(*width, *height, *n_colours);
        }
        if HFlip::applicable(*allowed_xforms, square) {
            orbits += HFlip::ORBITS;
            fixed += HFlip::total_fixed(*width, *height, *n_colours);
        }
        if VFlip::applicable(*allowed_xforms, square) {
            orbits += VFlip::ORBITS;
            fixed += VFlip::total_fixed(*width, *height, *n_colours);
        }
        if DFlip::applicable(*allowed_xforms, square) {
            orbits += DFlip::ORBITS;
            fixed += DFlip::total_fixed(*width, *height, *n_colours);
        }

        *perm_count = fixed / orbits;

        let transforms = Transforms::new(*width, *height, *allowed_xforms);
        if *perm_count <= 10000 {
            let mut seen = HashSet::with_capacity(*perm_count as usize);
            build_permutations(
                &transforms,
                *width,
                *height,
                *n_colours,
                0,
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
            width,
            height,
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
                    let (mut rect, _response) = ui.allocate_exact_size(
                        ((20 * *width) as f32, (20 * *height) as f32).into(),
                        egui::Sense::hover(),
                    );
                    rect.set_width(20.);
                    rect.set_height(20.);

                    for x in 0..*width {
                        for y in 0..*height {
                            let r = rect.translate((20. * x as f32, 20. * y as f32).into());
                            ui.painter().rect_filled(
                                r,
                                2.,
                                palette[permutation.get(x as usize, y as usize, *width as usize)
                                    as usize],
                            );
                        }
                    }
                }
            });
        }
    }

    pub fn export_json(&self) -> (String, Vec<u8>) {
        let Self { width, height, n_colours, allowed_xforms, permutations, ..} = self;
        let mut rv = vec![];
        for permutation in permutations {
            let mut entry = vec![];
            for y in 0..*height {
                let mut row = vec![];
                for x in 0..*width {
                    row.push(permutation.get(y as usize, x as usize, *width as usize));
                }
                entry.push(row);
            }
            rv.push(entry)
        }
        let name = format!("{}x{} {} col {:?}.json", width, height, n_colours, allowed_xforms);
        let bytes =serde_json::to_vec(&rv).unwrap();
        (name, bytes)
    }
}
