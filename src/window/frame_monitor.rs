
use std::collections::VecDeque;


pub struct FrameMonitor {

    start_time:          std::time::Instant,
    timing_queue:        VecDeque<std::time::Duration>,
    frame_average_count: usize
}


impl FrameMonitor {

    pub fn new() -> Self {

        Self {
            start_time:          std::time::Instant::now(),
            timing_queue:        VecDeque::<std::time::Duration>::new(),
            frame_average_count: 30
        }
    }


    pub fn start_frame(&mut self) {

        self.start_time = std::time::Instant::now();
    }


    pub fn end_frame(&mut self) {

        let duration = self.start_time.elapsed();

        self.timing_queue.push_back(duration);

        if self.timing_queue.len() > self.frame_average_count {

            self.timing_queue.pop_front();
        }
    }


    pub fn get_frame_time(&self) -> std::time::Duration {

        let mut total_duration = std::time::Duration::new(0, 0);

        for duration in &self.timing_queue {

            total_duration += *duration;
        }

        if self.timing_queue.len() == 0 {
            std::time::Duration::new(0, 0)
        } else {
            total_duration / self.timing_queue.len() as u32
        }
    }
}
