use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Time {
    last_frame: Instant,
    accumulator: Duration,
    fixed_delta: Duration,
    max_frame_delta: Duration,
}

impl Time {
    #[must_use]
    pub fn new(fixed_delta: Duration) -> Self {
        Self {
            last_frame: Instant::now(),
            accumulator: Duration::ZERO,
            fixed_delta,
            max_frame_delta: fixed_delta * 10,
        }
    }

    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        let frame_delta = now - self.last_frame;
        self.last_frame = now;

        self.accumulator += frame_delta.min(self.max_frame_delta);
    }

    #[must_use]
    pub const fn fixed_delta(&self) -> Duration {
        self.fixed_delta
    }

    pub fn clear_accumulator(&mut self) {
        self.accumulator = Duration::ZERO;
    }

    #[must_use]
    pub fn should_run_fixed_update(&self) -> bool {
        self.accumulator >= self.fixed_delta
    }

    pub fn consume_fixed_update(&mut self) {
        self.accumulator -= self.fixed_delta;
    }
}
