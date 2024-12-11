#[derive(Default)]
pub struct SystemMetrics {
    pub cpu_history: Vec<f32>,
    pub mem_history: Vec<f32>,
    pub max_points: usize,
}

impl SystemMetrics {
    pub fn new(max_points: usize) -> Self {
        Self {
            cpu_history: Vec::new(),
            mem_history: Vec::new(),
            max_points,
        }
    }

    pub fn add_measurement(&mut self, cpu: f32, mem: f32) {
        self.cpu_history.push(cpu);
        self.mem_history.push(mem);

        if self.cpu_history.len() > self.max_points {
            self.cpu_history.remove(0);
            self.mem_history.remove(0);
        }
    }
}
