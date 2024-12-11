use egui_plot::{Line, Plot, PlotPoints};
use std::time::Instant;
use sysinfo::{Pid, ProcessesToUpdate, System, MINIMUM_CPU_UPDATE_INTERVAL};

mod metrics;
mod theme;

use metrics::SystemMetrics;
use theme::MainTheme;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Sys Monitor",
        options,
        Box::new(|_cc| Ok(Box::new(SysApp::default()))),
    )
}

#[derive(Default)]
struct MemGbs {
    used: f32,
    total: f32,
}

#[derive(Clone)]
struct ProcessInfo {
    pid: Pid,
    name: String,
    cpu_usage: f32,
    mem_usage: f32,
}

// App Struct
struct SysApp {
    sys: System,
    metrics: SystemMetrics,
    cpu_usage: f32,
    core_usage: Vec<f32>,
    mem_usage: f32,
    mem_gbs: MemGbs,
    proc_list: Vec<ProcessInfo>,
    last_update: std::time::Instant,
}

impl Default for SysApp {
    fn default() -> Self {
        let mut sys = System::new_all();
        let core_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        sys.refresh_all();
        Self {
            sys,
            metrics: SystemMetrics::new(50),
            core_usage,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            mem_gbs: MemGbs::default(),
            proc_list: Vec::new(),
            last_update: Instant::now(),
        }
    }
}

// UI for updates
impl eframe::App for SysApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut theme = MainTheme::new(ctx);
        theme.set_theme();

        ctx.request_repaint();

        if self.last_update.elapsed()
            >= MINIMUM_CPU_UPDATE_INTERVAL + std::time::Duration::from_secs(1)
        {
            self.get_base_usage();
            self.last_update = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // general info
            ui.add_space(ui.available_height() * 0.01);
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(32))
                        .rounding(8.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100)))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading("CPU");
                                ui.label(format!("Total usage: {:.1}%", self.cpu_usage));
                                for (i, core) in self.core_usage.iter().enumerate() {
                                    ui.label(format!("Core {}: {:.1}%", i, core));
                                }
                            });
                        });

                    ui.add_space(8.0);

                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(32))
                        .rounding(8.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100)))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading("Memory");
                                ui.label(format!("Total usage: {:.1}%", self.mem_usage));
                                ui.label(format!(
                                    "Usage in GB: {:.1}GBs / {:.1}GBs",
                                    self.mem_gbs.used, self.mem_gbs.total
                                ));
                            });
                        });

                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(32))
                        .rounding(8.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100)))
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.heading("Metrics");
                                self.draw_graphs(ui);
                            });
                        });
                });
            });

            // Process List
            egui::Frame::none()
                .fill(egui::Color32::from_gray(32))
                .rounding(8.0)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(100)))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.heading(format!("Processes ({})", self.proc_list.len()));
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("process_grid")
                            .num_columns(4)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                // headers
                                ui.strong("Name");
                                ui.strong("PID");
                                ui.strong("CPU Usage");
                                ui.strong("Memory Usage");
                                ui.end_row();

                                for process in self.proc_list.iter() {
                                    ui.label(process.name.clone());
                                    ui.label(process.pid.to_string());
                                    ui.label("N/A");
                                    ui.label(format!("{:.1}MBs", process.mem_usage));
                                    ui.end_row();
                                }
                            });
                    });
                });
        });
    }
}

// helper functions
impl SysApp {
    fn get_base_usage(&mut self) {
        // refreshing
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        self.core_usage = self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        self.cpu_usage = self.core_usage.iter().sum::<f32>() / self.core_usage.len() as f32;

        let used = self.sys.used_memory();
        let total = self.sys.total_memory();
        let used_gbs = used as f32 / (1024.0 * 1024.0 * 1024.0);
        let total_gbs = total as f32 / (1024.0 * 1024.0 * 1024.0);
        self.mem_gbs.used = used_gbs;
        self.mem_gbs.total = total_gbs;
        self.mem_usage = (used as f32 / total as f32) * 100.0;

        self.proc_list = self
            .sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessInfo {
                pid: *pid,
                name: proc.name().to_string_lossy().to_string(),
                cpu_usage: proc.cpu_usage() * 100.0,
                mem_usage: proc.memory() as f32 / (1024.0 * 1024.0),
            })
            .collect();

        self.proc_list.sort_by(|a, b| {
            b.mem_usage
                .partial_cmp(&a.mem_usage)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.metrics.add_measurement(self.cpu_usage, self.mem_usage);
    }

    pub fn draw_graphs(&self, ui: &mut egui::Ui) {
        Plot::new("System Metrics")
            .height(200.0)
            .show(ui, |plot_ui| {
                let cpu_points: PlotPoints = self
                    .metrics
                    .cpu_history
                    .iter()
                    .enumerate()
                    .map(|(i, &cpu)| [i as f64, cpu as f64])
                    .collect();
                plot_ui.line(Line::new(cpu_points).name("CPU Usage"));

                let mem_points: PlotPoints = self
                    .metrics
                    .mem_history
                    .iter()
                    .enumerate()
                    .map(|(i, &mem)| [i as f64, mem as f64])
                    .collect();
                plot_ui.line(Line::new(mem_points).name("Memory Usage"));
            });
    }
}
