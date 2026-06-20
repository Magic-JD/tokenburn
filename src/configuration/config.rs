use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub frames_per_second: i8,
    pub spread_over_seconds: i16,
}

impl Config {
    fn default() -> Self {
        Self {
            frames_per_second: 12,
            spread_over_seconds: 60,
        }
    }

    pub fn get_config() -> &'static Config {
        CONFIG.get_or_init(Config::default)
    }
}
