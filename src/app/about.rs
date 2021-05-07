use eframe::egui;

pub struct About {
    open: bool,
}
impl About {
    pub fn new() -> Self {
        About { open: false }
    }
    pub fn open(&mut self) {
        self.open = true;
    }
    pub fn show(&mut self, ctx: &egui::CtxRef) {
        egui::Window::new("Help / About").open(&mut self.open).default_width(400.).scroll(true).show(ctx, |ui| {
            ui.label("Burnside Generator");
            ui.label("Mark Sherry © 2021");
            ui.separator();

            ui.label("Given a necklace with beads that can rotate freely around the string, and N colours of beads, how many distinct necklaces are there?");
            ui.label("Given a rectangular X×Y game tile with N possible markings in each position, how many distinct tiles are there? What if you can rotate or flip the tiles?");
            ui.label("Burnside's Lemma is a mathematical result that provides a tool to answer these, and similar questions.");
            ui.label("This calculator aims to be a useful tool for game designers, and those just curious about distinct combinations.");
            ui.separator();
            ui.label("For more information about the lemma, see:");
            ui.hyperlink_to("Burnside's Lemma", "https://en.wikipedia.org/wiki/Burnside%27s_lemma");
        });
    }
}
