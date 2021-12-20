use std::time::Instant;

use humantime::format_duration;

pub struct Timer(Instant);

impl Timer {
    pub fn start() -> Self {
        Self(Instant::now())
    }

    pub fn stop(self) -> String {
        format_duration(self.0.elapsed()).to_string()
    }
}
