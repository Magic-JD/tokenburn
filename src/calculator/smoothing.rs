use crate::configuration::config::Config;

pub struct Smoothing {
    token_smoothing: Vec<f32>,
    len: usize,
    spread_over_seconds: i16,
}

impl Smoothing {
    pub fn new() -> Self {
        let config = Config::get_config();
        let fps = config.frames_per_second;
        let seconds_spread = config.spread_over_seconds;
        let size = seconds_spread as usize * fps as usize;
        Smoothing {
            token_smoothing: vec![0f32; size],
            len: size,
            spread_over_seconds: seconds_spread,
        }
    }

    pub fn update_and_retrieve(&mut self, cost_per_second: f32) -> f32 {
        self.insert_smoothing(cost_per_second);
        self.retrieve_smoothing()
    }

    fn insert_smoothing(&mut self, cost_per_second: f32) {
        if cost_per_second != 0f32 {
            let averaged_cost_per_second = cost_per_second / self.spread_over_seconds as f32;
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
