
use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use web_time::{Instant, Duration};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, Duration};


pub struct FrameMonitor {

    start_time:          Instant,
    timing_queue:        VecDeque<std::time::Duration>,
    frame_average_count: usize
}


impl FrameMonitor {

    pub fn new() -> Self {

        Self {
            start_time:          Instant::now(),
            timing_queue:        VecDeque::<std::time::Duration>::new(),
            frame_average_count: 30
        }
    }


    pub fn start_frame(&mut self) {

        self.start_time = Instant::now();
    }


    pub fn end_frame(&mut self) {

        let duration = self.start_time.elapsed();

        self.timing_queue.push_back(duration);

        if self.timing_queue.len() > self.frame_average_count {

            self.timing_queue.pop_front();
        }
    }


    pub fn get_frame_time(&self) -> Duration {

        let mut total_duration = Duration::new(0, 0);

        for duration in &self.timing_queue {

            total_duration += *duration;
        }

        if self.timing_queue.len() == 0 {
            Duration::new(0, 0)
        } else {
            total_duration / self.timing_queue.len() as u32
        }
    }
}
