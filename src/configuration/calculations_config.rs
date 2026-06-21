use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CalculationsConfig {
    pub percentage_ramp: Option<i16>,
    pub spread_in_seconds: Option<i16>,
}
