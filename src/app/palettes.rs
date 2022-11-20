use eframe::egui::{self, Color32, CtxRef, Sense, Ui};
pub type Palette = [Color32; 8];
pub static A: Palette = [
    Color32::from_rgb(127, 201, 127),
    Color32::from_rgb(190, 174, 212),
    Color32::from_rgb(253, 192, 134),
    Color32::from_rgb(255, 255, 153),
    Color32::from_rgb(56, 108, 176),
    Color32::from_rgb(240, 2, 127),
    Color32::from_rgb(191, 91, 23),
    Color32::from_rgb(102, 102, 102),
];

pub static B: Palette = [
    Color32::from_rgb(27, 158, 119),
    Color32::from_rgb(217, 95, 2),
    Color32::from_rgb(117, 112, 179),
    Color32::from_rgb(231, 41, 138),
    Color32::from_rgb(102, 166, 30),
    Color32::from_rgb(230, 171, 2),
    Color32::from_rgb(166, 118, 29),
    Color32::from_rgb(102, 102, 102),
];

pub static C: Palette = [
    Color32::from_rgb(166, 206, 227),
    Color32::from_rgb(31, 120, 180),
    Color32::from_rgb(178, 223, 138),
    Color32::from_rgb(51, 160, 44),
    Color32::from_rgb(251, 154, 153),
    Color32::from_rgb(227, 26, 28),
    Color32::from_rgb(253, 191, 111),
    Color32::from_rgb(255, 127, 0),
];

pub static D: Palette = [
    Color32::from_rgb(251, 180, 174),
    Color32::from_rgb(179, 205, 227),
    Color32::from_rgb(204, 235, 197),
    Color32::from_rgb(222, 203, 228),
    Color32::from_rgb(254, 217, 166),
    Color32::from_rgb(255, 255, 204),
    Color32::from_rgb(229, 216, 189),
    Color32::from_rgb(253, 218, 236),
];

pub static E: Palette = [
    Color32::from_rgb(179, 226, 205),
    Color32::from_rgb(253, 205, 172),
    Color32::from_rgb(203, 213, 232),
    Color32::from_rgb(244, 202, 228),
    Color32::from_rgb(230, 245, 201),
    Color32::from_rgb(255, 242, 174),
    Color32::from_rgb(241, 226, 204),
    Color32::from_rgb(204, 204, 204),
];

pub static F: Palette = [
    Color32::from_rgb(228, 26, 28),
    Color32::from_rgb(55, 126, 184),
    Color32::from_rgb(77, 175, 74),
    Color32::from_rgb(152, 78, 163),
    Color32::from_rgb(255, 127, 0),
    Color32::from_rgb(255, 255, 51),
    Color32::from_rgb(166, 86, 40),
    Color32::from_rgb(247, 129, 191),
];

pub static G: Palette = [
    Color32::from_rgb(102, 194, 165),
    Color32::from_rgb(252, 141, 98),
    Color32::from_rgb(141, 160, 203),
    Color32::from_rgb(231, 138, 195),
    Color32::from_rgb(166, 216, 84),
    Color32::from_rgb(255, 217, 47),
    Color32::from_rgb(229, 196, 148),
    Color32::from_rgb(179, 179, 179),
];

pub static H: Palette = [
    Color32::from_rgb(141, 211, 199),
    Color32::from_rgb(255, 255, 179),
    Color32::from_rgb(190, 186, 218),
    Color32::from_rgb(251, 128, 114),
    Color32::from_rgb(128, 177, 211),
    Color32::from_rgb(253, 180, 98),
    Color32::from_rgb(179, 222, 105),
    Color32::from_rgb(252, 205, 229),
];

pub struct PaletteChooser {
    pub choice: &'static Palette,
    visible: bool,
}

impl PaletteChooser {
    pub fn new() -> Self {
        PaletteChooser {
            choice: &A,
            visible: false,
        }
    }

    fn render_palette(ui: &mut Ui, palette: &Palette) {
        let (mut rect, _response) = ui.allocate_exact_size((160., 20.).into(), Sense::hover());
        rect.set_width(20.);
        #[allow(clippy::needless_range_loop)]
        for i in 0..8 {
            let offset = (20 * i) as f32;
            ui.painter()
                .rect_filled(rect.translate((offset, 0.).into()), 1., palette[i]);
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
    }

    pub fn choose(&mut self, ctx: &CtxRef) {
        let PaletteChooser { visible, choice } = self;
        if *visible {
            egui::Window::new("Palette picker")
                .open(visible)
                .auto_sized()
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &A, "Palette A");
                        Self::render_palette(ui, &A);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &B, "Palette B");
                        Self::render_palette(ui, &B);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &C, "Palette C");
                        Self::render_palette(ui, &C);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &D, "Palette D");
                        Self::render_palette(ui, &D);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &E, "Palette E");
                        Self::render_palette(ui, &E);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &F, "Palette F");
                        Self::render_palette(ui, &F);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &G, "Palette G");
                        Self::render_palette(ui, &G);
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(choice, &H, "Palette H");
                        Self::render_palette(ui, &H);
                    });
                });
        }
    }
}
