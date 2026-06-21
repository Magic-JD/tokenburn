use crate::configuration::config::Config;

pub struct Smoothing {
    token_smoothing: Vec<f32>,
    len: usize,
    spread_over_seconds: i16,
    ramp_frames: usize,
}

impl Smoothing {
    pub fn new() -> Self {
        let config = Config::get_config();
        let fps = config.frames_per_second;
        let seconds_spread = config.time_period_seconds;
        let base_frames_needed = seconds_spread as usize * fps as usize;
        let ramp_frames = ((base_frames_needed as f32 / 100f32) * config.percent_ramp as f32) as usize;
        let ramp_adjusted_frames_needed = base_frames_needed + ramp_frames;
        Smoothing {
            token_smoothing: vec![0f32; ramp_adjusted_frames_needed],
            len: ramp_adjusted_frames_needed,
            spread_over_seconds: seconds_spread,
            ramp_frames,
        }
    }

    pub fn update_and_retrieve(&mut self, cost_per_second: f32) -> f32 {
        self.insert_smoothing(cost_per_second);
        self.retrieve_smoothing()
    }

    fn insert_smoothing(&mut self, cost_per_second: f32) {
        if cost_per_second != 0f32 {
            let averaged_cost_per_second = cost_per_second / self.spread_over_seconds as f32;
            for i in 0..self.ramp_frames {
                let fractional = averaged_cost_per_second / (self.ramp_frames - 1) as f32;
                self.token_smoothing[i] += i as f32 * fractional;
            }

            for i in self.ramp_frames..self.len - self.ramp_frames {
                self.token_smoothing[i] += averaged_cost_per_second;
            }

            for i in self.len - self.ramp_frames..self.len {
                let fractional = averaged_cost_per_second / (self.ramp_frames - 1) as f32;
                let idx = self.len - (i + 1);
                self.token_smoothing[i] += idx as f32 * fractional;
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
