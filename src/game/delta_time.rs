pub struct DeltaTime {
    previous_time: f64,
    total_time: f64,
    delta: f64,
}

impl DeltaTime {
    pub fn new(previous: f64) -> Self {
        Self {
            previous_time: previous,
            total_time: 0.0,
            delta: 0.0,
        }
    }

    pub fn update(&mut self, current_time: f64) {
        self.delta = current_time - self.previous_time;
        self.total_time += self.delta;
        self.previous_time = current_time;
    }

    pub fn get(&self) -> f32 {
        self.delta as f32
    }

    pub fn total(&self) -> f32 {
        self.total_time as f32
    }
}
