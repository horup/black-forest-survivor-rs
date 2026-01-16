#[derive(Default, Clone)]
pub struct Timer {
    duration_sec: f32,
    elapsed: f32,
    finished: bool,
}

impl Timer {
    pub fn new(duration_sec: f32, finished: bool) -> Self {
        Self {
            duration_sec,
            elapsed: 0.0,
            finished,
        }
    }

    pub fn tick(&mut self, delta: f32) {
        if !self.finished {
            self.elapsed += delta;
            if self.elapsed >= self.duration_sec {
                self.finished = true;
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn restart(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration_sec).min(1.0)
    }
}