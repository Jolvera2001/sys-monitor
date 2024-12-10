pub struct MainTheme<'a> {
    ctx: &'a egui::Context,
}

impl<'a> MainTheme<'a> {
    pub fn new(ctx: &'a egui::Context) -> Self {
        Self { ctx }
    }

    pub fn set_theme(&mut self) {
        let mut style = (*self.ctx.style()).clone();

        use ::std::collections::BTreeMap;
        style.text_styles = [
            (
                egui::TextStyle::Body,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(16.0, egui::FontFamily::Monospace),
            ),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();

        self.ctx.set_style(style);
    }
}
