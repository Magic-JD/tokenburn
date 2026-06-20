use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub frames_per_second: i64,
}

impl Config {
    fn default() -> Self {
        Self { frames_per_second: 12}
    }

    pub fn get_config() -> &'static Config {
        CONFIG.get_or_init(Config::default)
    }
}