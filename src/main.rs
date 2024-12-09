use std::time::Instant;
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};

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

// App Struct
struct SysApp {
    last_updated: std::time::Instant,
    sys: System,
    cpu_usage: f32
}

impl Default for SysApp {
    fn default() -> Self {
        Self {
            last_updated: Instant::now(),
            sys: System::new_all(),
            cpu_usage: 0.0,
        }
    }
}

// UI for updates
impl eframe::App for SysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // so the ui updates at this interval
        ctx.request_repaint_after(MINIMUM_CPU_UPDATE_INTERVAL);

        // to make sure we have a starting update to draw from (documentation says so)
        self.get_cpu_usage();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("CPU: {:.1}%", self.cpu_usage));
            });
        });
    }
}

// helper functions
impl SysApp {
    fn get_cpu_usage(&mut self) {
        if self.should_update() { // if enough time has passed
            self.last_updated = Instant::now();
            self.sys.refresh_cpu_usage();
            let mut total = 0.0;
            for cpu in self.sys.cpus() {
                total += cpu.cpu_usage();
            }
            self.cpu_usage = total / self.sys.cpus().len() as f32
        }
    }

    fn should_update(&mut self) -> bool {
        self.last_updated.elapsed() >= MINIMUM_CPU_UPDATE_INTERVAL
    }
}