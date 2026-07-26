
use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use web_time::{Instant, Duration};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, Duration};


pub struct FrameMonitor {

    draw_call_start_time:           Option<Instant>,
    surface_acquisition_start_time: Option<Instant>,
    render_start_time:              Option<Instant>,
    submit_start_time:              Option<Instant>,
    time_between_draws:             TimingQueue,
    surface_acquisition_time:       TimingQueue,
    surface_discard_time:           TimingQueue,
    render_time:                    TimingQueue,
    submit_time:                    TimingQueue
}


impl FrameMonitor {

    #[must_use]
    pub fn new(av_count: usize) -> Self {

        Self {
            draw_call_start_time:           None,
            surface_acquisition_start_time: None,
            render_start_time:              None,
            submit_start_time:              None,
            time_between_draws:             TimingQueue::new(av_count),
            surface_acquisition_time:       TimingQueue::new(av_count),
            surface_discard_time:           TimingQueue::new(av_count),
            render_time:                    TimingQueue::new(av_count),
            submit_time:                    TimingQueue::new(av_count)
        }
    }


    pub fn start_draw_call(&mut self) {

        if let Some(time) = self.draw_call_start_time {

            self.time_between_draws.add_time(time.elapsed());
        }

        self.draw_call_start_time = Some(Instant::now());
    }


    pub fn start_surface_acquisition(&mut self) {

        self.surface_acquisition_start_time = Some(Instant::now());
    }


    pub fn surface_acquired(&mut self) {

        let Some(time) = self.surface_acquisition_start_time else { return; };
        self.surface_acquisition_time.add_time(time.elapsed());
        self.surface_acquisition_start_time = None;
    }


    pub fn surface_discarded(&mut self) {

        let Some(time) = self.surface_acquisition_start_time else { return; };
        self.surface_discard_time.add_time(time.elapsed());
        self.surface_acquisition_start_time = None;
    }


    pub fn start_rendering(&mut self) {

        self.render_start_time = Some(Instant::now());
    }


    pub fn end_rendering(&mut self) {

        let Some(time) = self.render_start_time else { return; };
        self.render_time.add_time(time.elapsed());
        self.render_start_time = None;
    }


    pub fn start_submitting(&mut self) {

        self.submit_start_time = Some(Instant::now());
    }


    pub fn end_submitting(&mut self) {

        let Some(time) = self.submit_start_time else { return ;};
        self.submit_time.add_time(time.elapsed());
        self.submit_start_time = None;
    }


    #[must_use]
    pub fn get_time_between_draws(&self) -> Option<Duration> {

        self.time_between_draws.get_average()
    }


    #[must_use]
    pub fn get_surface_acquisition_time(&self) -> Option<Duration> {

        self.surface_acquisition_time.get_average()
    }


    #[must_use]
    pub fn get_surface_discard_time(&self) -> Option<Duration> {

        self.surface_discard_time.get_average()
    }


    #[must_use]
    pub fn get_render_time(&self) -> Option<Duration> {

        self.render_time.get_average()
    }


    #[must_use]
    pub fn get_submit_time(&self) -> Option<Duration> {

        self.submit_time.get_average()
    }


    #[must_use]
    pub fn get_fps(&self) -> Option<f32> {

        let time = self.get_time_between_draws()?;
        Some(1.0 / time.as_secs_f32())
    }
}


struct TimingQueue {

    queue: VecDeque<std::time::Duration>,
    size:  usize
}


impl TimingQueue {


    #[must_use]
    pub fn new(size: usize) -> Self {

        Self {
            queue: VecDeque::<std::time::Duration>::with_capacity(size),
            size
        }
    }


    pub fn add_time(&mut self, time: std::time::Duration) {

        if self.queue.len() == self.size {
            self.queue.pop_front();
        }

        self.queue.push_back(time);
    }


    #[must_use]
    pub fn get_average(&self) -> Option<Duration> {

        if self.queue.is_empty() {
            None
        } else {
            let total_time: Duration = self.queue.iter().sum();
            Some(total_time / self.queue.len() as u32)
        }
    }
}
