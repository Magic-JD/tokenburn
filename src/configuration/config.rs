use crate::cli::command::ConfigArgs;
use crate::configuration::calculations_config::CalculationsConfig;
use crate::utils::file_system::get_user_base_config_file;
use serde::Deserialize;
use std::sync::OnceLock;
use toml;

static CONFIG: OnceLock<Config> = OnceLock::new();
pub const DEFAULT_CONFIG: &str = include_str!("config.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub frames_per_second: i8,
    pub time_period_seconds: i16,
    pub percent_ramp: i16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawConfig {
    #[serde(default)]
    pub calculations: Option<CalculationsConfig>,
}

impl Config {
    pub fn init(config_args: ConfigArgs) {
        let config = Config::new(config_args.ramp, config_args.spread);
        CONFIG
            .set(config)
            .expect("Failed to set configuration due to preexisting configuration");
    }

    fn new(ramp: Option<i16>, spread: Option<i16>) -> Self {
        let default: RawConfig =
            toml::from_str(DEFAULT_CONFIG).expect("Tool defined configuration failing to compile");
        let user_defined_calculation_config: Option<CalculationsConfig> =
            get_user_base_config_file()
                .and_then(|str| toml::from_str(&str).ok())
                .and_then(|tool_config: RawConfig| tool_config.calculations);
        Self {
            frames_per_second: 12,
            time_period_seconds: spread.unwrap_or(Self::extract_required(
                &default,
                &user_defined_calculation_config,
                &|conf| conf.spread_in_seconds,
            )),
            percent_ramp: ramp.unwrap_or(Self::extract_required(
                &default,
                &user_defined_calculation_config,
                &|conf| conf.percentage_ramp,
            )),
        }
    }

    fn extract_required(
        default: &RawConfig,
        user_defined: &Option<CalculationsConfig>,
        extraction_function: &dyn Fn(CalculationsConfig) -> Option<i16>,
    ) -> i16 {
        user_defined
            .clone()
            .and_then(extraction_function)
            .unwrap_or(
                default
                    .clone()
                    .calculations
                    .and_then(extraction_function)
                    .expect("Required configuration must be defined on the base config."),
            )
    }

    pub fn get_config() -> &'static Config {
        CONFIG.get().expect("Configuration is not initialized")
    }
}
