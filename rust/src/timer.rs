use std::time::{Duration, Instant};

pub struct Timer {
    total: Duration,
    aux: Instant,
}

impl Timer {
    pub fn novo() -> Self {
        Timer {
            total: Duration::new(0, 0),
            aux: Instant::now()
        }
    }

    pub fn iniciar(&mut self) {
        self.aux = Instant::now();
    }

    pub fn parar(&mut self) {
        self.total += self.aux.elapsed();
    }

    pub fn total(&self) -> f64 {
        return self.total.as_secs_f64();
    }
}