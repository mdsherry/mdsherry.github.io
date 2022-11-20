use eframe::egui;

mod about;
mod bag_draw;
mod palettes;
mod ring;
mod tile;
use self::palettes::PaletteChooser;
use about::About;
use ring::Ring;
use tile::Tile;

mod downloader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedMode {
    Tile,
    Ring,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BurnsideApp {
    mode: SelectedMode,
    tile: Tile,
    ring: Ring,
    palette: PaletteChooser,
    about: About,
}

impl BurnsideApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl Default for BurnsideApp {
    fn default() -> Self {
        Self {
            mode: SelectedMode::Tile,
            tile: Tile::new(),
            ring: Ring::new(),
            palette: PaletteChooser::new(),
            about: About::new(),
        }
    }
}

impl eframe::App for BurnsideApp {
    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let BurnsideApp {
            mode,
            tile,
            ring,
            palette,
            about,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::bar(ui, |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    if ui.button("Palette").clicked() {
                        palette.open();
                    }
                    if ui.button("Help / about").clicked() {
                        about.open();
                    }
                });
            });
        });
        palette.choose(ctx);
        about.show(ctx);
        let mut changed = false;
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.radio_value(mode, SelectedMode::Tile, "Tiles");
            ui.radio_value(mode, SelectedMode::Ring, "Rings");
            changed = match mode {
                SelectedMode::Tile => tile.settings(ui),
                SelectedMode::Ring => ring.settings(ui),
            };
        });

        if changed {
            match *mode {
                SelectedMode::Tile => tile.recompute_perms(),
                SelectedMode::Ring => ring.recompute_perms(),
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| match *mode {
                    SelectedMode::Tile => tile.render_results(palette.choice, ui),
                    SelectedMode::Ring => ring.render_results(palette.choice, ui),
                });
        });
    }
}

// ----------------------------------------------------------------------------
