use crate::configuration::config::Config;

const SECONDS_IN_MINUTE: i64 = 60;

pub struct Smoothing {
    token_smoothing: Vec<f32>,
    len: usize,
}

impl Smoothing {
    pub fn new() -> Self {
        let fps = Config::get_config().frames_per_second;
        let size = (SECONDS_IN_MINUTE * fps) as usize;
        Smoothing { token_smoothing: vec![0f32; size], len: size, }
    }

    pub fn smooth_per_second(&mut self, cost_per_second: f32) -> f32 {
        self.insert_smoothing(cost_per_second);
        self.retrieve_smoothing()
    }

    fn insert_smoothing(&mut self, cost_per_second: f32) {
        if cost_per_second != 0f32 {
            let averaged_cost_per_second = cost_per_second / 60f32;
            for i in 0..self.len {
                self.token_smoothing[i] += averaged_cost_per_second;
            }
        }
    }

    fn retrieve_smoothing(&mut self) -> f32 {
        let current_smoothed = self.token_smoothing[0];
        for i in 0..self.len - 1 {
            self.token_smoothing[i] = self.token_smoothing[i + 1];
        }
        self.token_smoothing[self.len - 1] = 0f32;
        current_smoothed
    }
}
