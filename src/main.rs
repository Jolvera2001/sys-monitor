use std::time::Instant;
use sysinfo::{System, MINIMUM_CPU_UPDATE_INTERVAL};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 450.0]),
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
    last_cpu_update: std::time::Instant,
    last_mem_update: std::time::Instant,
    sys: System,
    cpu_usage: f32,
    mem_usage: f32,
}

impl Default for SysApp {
    fn default() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all(); 
        Self {
            last_cpu_update: Instant::now(),
            last_mem_update: Instant::now(),
            sys,
            cpu_usage: 0.0,
            mem_usage: 0.0,
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
        self.get_mem_usage();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(format!("CPU: {:.1}%", self.cpu_usage));
                ui.heading(format!("Memory: {:.1}%", self.mem_usage));
            });
        });
    }
}

// helper functions
impl SysApp {
    fn get_cpu_usage(&mut self) {
        if self.should_update_cpu() { // if enough time has passed
            self.last_cpu_update = Instant::now();
            self.sys.refresh_cpu_usage();
            let mut total = 0.0;
            for cpu in self.sys.cpus() {
                total += cpu.cpu_usage();
            }
            self.cpu_usage = total / self.sys.cpus().len() as f32
        }
    }

    fn get_mem_usage(&mut self) {
        if self.should_update_mem() {
            self.last_mem_update = Instant::now();
            self.sys.refresh_memory();
            let used = self.sys.used_memory();
            let total = self.sys.total_memory();
            // println!("Used: {}, Total: {}", used, total); // Debug
            self.mem_usage = (used as f32 / total as f32) * 100.0
        }
    }

    fn should_update_cpu(&mut self) -> bool {
        self.last_cpu_update.elapsed() >= MINIMUM_CPU_UPDATE_INTERVAL
    }

    fn should_update_mem(&mut self) -> bool {
        self.last_mem_update.elapsed() >= MINIMUM_CPU_UPDATE_INTERVAL
    }
}