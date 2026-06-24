use std::time::SystemTime;
use crate::calculator::log_tracker::LogTracker;
use crate::calculator::price_calculator::PriceCalculator;
use crate::calculator::smoothing::Smoothing;

pub struct Calculator {
    price_calc: PriceCalculator,
    log_tracker: LogTracker,
    smoothing: Smoothing,
    last_checked: SystemTime,
}
impl Calculator {
    pub fn new() -> Self {
        Self {
            price_calc: PriceCalculator::new(),
            log_tracker: LogTracker::new(),
            smoothing: Smoothing::new(),
            last_checked: SystemTime::now(),
        }
    }

    pub fn current_cost_per_minute(&mut self) -> f32 {
        let time = SystemTime::now();
        if time.duration_since(self.last_checked).unwrap().as_millis() < 1000 {
            return self.smoothing.update_and_retrieve(0.0) * 60f32
        }
        self.last_checked = time;
        let cost_this_sec = self
            .log_tracker
            .new_logs()
            .into_iter()
            .map(|message| {
                self.price_calc
                    .calculate_cost(&message.model, message.usage)
            })
            .sum();
        self.smoothing.update_and_retrieve(cost_this_sec) * 60f32
    }
}
