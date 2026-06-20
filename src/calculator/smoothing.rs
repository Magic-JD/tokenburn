pub struct Smoothing {
    token_smoothing: [f32; 10],
}

impl Smoothing {
    pub fn new() -> Self { Smoothing { token_smoothing: [0f32; 10] } }

    pub fn smooth_per_second(&mut self, cost_per_second: f32) -> f32 {
        self.insert_smoothing(cost_per_second);
        self.retrieve_smoothing()
    }

    fn insert_smoothing(&mut self, cost_per_second: f32) {
        let mut cost_per_second = cost_per_second;
        if cost_per_second != 0f32 {
            cost_per_second /= 2f32;
            let division_factor = 2f32;
            let center = self.token_smoothing.len() / 2;
            for i in 0..center - 1 {
                cost_per_second /= division_factor;
                self.token_smoothing[center - i - 1] += cost_per_second;
                self.token_smoothing[center + i] += cost_per_second;
            }
            self.token_smoothing[0] += cost_per_second;
            self.token_smoothing[self.token_smoothing.len() - 1] += cost_per_second;
        }
    }

    fn retrieve_smoothing(&mut self) -> f32 {
        let current_smoothed = self.token_smoothing[0];
        for i in 0..self.token_smoothing.len() - 1 {
            self.token_smoothing[i] = self.token_smoothing[i + 1];
        }
        self.token_smoothing[self.token_smoothing.len() - 1] = 0f32;
        current_smoothed
    }
}