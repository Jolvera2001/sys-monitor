fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
            ..Default::default()
    };

    eframe::run_native(
        "Sys Monitor",
        options,
        Box::new(|cc| {
            Ok(Box::new(SysApp::default()))
        }),
    )
}

struct SysApp {
    cpu_usage: f32,
    mem_usage: f32,
}

impl Default for SysApp {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            mem_usage: 0.0
        }
    }
}

impl eframe::App for SysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Wowzers");
            });
        });
    }
}