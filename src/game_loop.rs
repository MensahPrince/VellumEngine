// src/game_loop.rs
use std::time::{Instant, Duration};

pub struct GameLoop {
    last_update: Instant,
    accumulated_time: Duration,
    update_rate: Duration, // Time per update (e.g., 1/60th of a second)
}

impl GameLoop {
    pub fn new(updates_per_second: f64) -> Self {
        Self {
            last_update: Instant::now(),
            accumulated_time: Duration::ZERO,
            update_rate: Duration::from_secs_f64(1.0 / updates_per_second),
        }
    }

    pub fn tick(&mut self) -> (f64, u32) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;
        self.accumulated_time += delta_time;

        let mut update_count = 0;
        while self.accumulated_time >= self.update_rate {
            self.accumulated_time -= self.update_rate;
            update_count += 1;
        }

        (delta_time.as_secs_f64(), update_count)
    }
}