use std::time::Instant;

#[derive(Clone, Copy)]
pub struct TimeManagment {
    time_left: u128,
    increment: u128,
    start_time: Instant,
    moves_to_go: Option<u64>,
}

impl TimeManagment {
    pub fn new(time_left: u128, increment: u128, moves_to_go: Option<u64>) -> TimeManagment {
        TimeManagment {
            time_left,
            increment,
            start_time: Instant::now(),
            moves_to_go,
        }
    }

    pub fn has_time_for_next_iteration(&self, last_iteration_time: u128) -> bool {
        let moves_to_go = self.moves_to_go.unwrap_or(24);
        let elapsed = self.start_time.elapsed().as_millis();
        let allocated_time = (self.time_left + self.increment) / moves_to_go as u128;
        let expected_time = last_iteration_time * 5;

        allocated_time > elapsed + expected_time
    }
}
