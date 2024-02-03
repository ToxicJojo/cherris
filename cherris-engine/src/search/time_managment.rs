use std::{time::Instant, u128};

pub struct TimeManagment {
    time_left: u64,
    increment: u64,
    start_time: Instant,
}

impl TimeManagment {
    pub fn new(time_left: u64, increment: u64) -> TimeManagment {
        TimeManagment {
            time_left,
            increment,
            start_time: Instant::now(),
        }
    }

    pub fn has_time_for_next_iteration(&self, last_iteration_time: u128) -> bool {
        let elapsed = self.start_time.elapsed().as_millis();
        let allocated_time = ((self.time_left + self.increment) / 24) as u128;
        let expected_time = last_iteration_time * 5;

        allocated_time > elapsed + expected_time
    }
}
