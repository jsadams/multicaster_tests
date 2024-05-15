use std::time::{Duration, Instant};

//pub ElapsedTimeTracker

pub struct ElapsedTimeTracker {
    last_time: Option<Instant>,
}

impl ElapsedTimeTracker {
    pub fn new() -> Self {
        Self { last_time: None }
    }

    pub fn get_elapsed(&mut self) -> Option<Duration> {
        let now = Instant::now();
        let elapsed = self.last_time.map(|t| now.duration_since(t));
        self.last_time = Some(now);
        elapsed
    }
}