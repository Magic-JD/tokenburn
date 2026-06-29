use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CalculationsConfig {
    pub percentage_ramp: Option<u32>,
    pub spread_in_seconds: Option<u32>,
    pub per_x_minutes: Option<u32>,
}
