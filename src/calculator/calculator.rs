use crate::calculator::log_tracker::LogTracker;
use crate::calculator::price_calculator::PriceCalculator;
use crate::calculator::smoothing::Smoothing;
use crate::configuration::config::Config;

pub struct Calculator {
    price_calc: PriceCalculator,
    log_tracker: LogTracker,
    smoothing: Smoothing,
}
impl Calculator {
    pub fn new() -> Self {
        Self {
            price_calc: PriceCalculator::new(),
            log_tracker: LogTracker::new(),
            smoothing: Smoothing::new(),
        }
    }

    pub fn current_cost_per_minute(&mut self) -> f32 {
        let cost_this_sec = self
            .log_tracker
            .new_logs()
            .into_iter()
            .map(|message| {
                self.price_calc
                    .calculate_cost(&message.model, message.usage)
            })
            .sum();
        self.smoothing.update_and_retrieve(cost_this_sec) * Config::get_config().per_x_seconds as f32
    }
}
