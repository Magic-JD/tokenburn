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
    pub time_period_seconds: u32,
    pub per_x_seconds: u32,
    pub percent_ramp: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawConfig {
    #[serde(default)]
    pub calculations: Option<CalculationsConfig>,
}

impl Config {
    pub fn init(config_args: ConfigArgs) {
        let config = Config::new(config_args.ramp, config_args.spread, config_args.per);
        CONFIG
            .set(config)
            .expect("Failed to set configuration due to preexisting configuration");
    }

    fn new(ramp: Option<u32>, spread: Vec<String>, per: Vec<String>) -> Self {
        let default: RawConfig =
            toml::from_str(DEFAULT_CONFIG).expect("Tool defined configuration failing to compile");
        let user_defined_calculation_config: Option<CalculationsConfig> =
            get_user_base_config_file()
                .and_then(|str| toml::from_str(&str).ok())
                .and_then(|tool_config: RawConfig| tool_config.calculations);
        Self {
            frames_per_second: 12,
            time_period_seconds: Self::extract_per_x_seconds(spread).unwrap_or(Self::extract_required(
                &default,
                &user_defined_calculation_config,
                &|conf| conf.spread_in_seconds,
            )),
            percent_ramp: ramp.unwrap_or(Self::extract_required(
                &default,
                &user_defined_calculation_config,
                &|conf| conf.percentage_ramp,
            )),
            per_x_seconds: Self::extract_per_x_seconds(per).unwrap_or(Self::extract_required(&default, &user_defined_calculation_config, &|conf|conf.per_x_seconds)),
        }
    }

    fn extract_per_x_seconds(all_requested: Vec<String>) -> Option<u32> {
        if all_requested.is_empty() {
            return None;
        }
        let sum = all_requested.into_iter().map(|requested| {
            let (amount, time_frame) = requested.split_at(requested.len() - 1);
            let total = amount.parse::<u32>().unwrap_or(1);
            match time_frame {
                "h" => total * 60 * 60,
                "m" => total * 60,
                "s" => total,
                _ => panic!("Invalid time frame"),
            }
        }).sum();
        Some(sum)
    }

    fn extract_required(
        default: &RawConfig,
        user_defined: &Option<CalculationsConfig>,
        extraction_function: &dyn Fn(CalculationsConfig) -> Option<u32>,
    ) -> u32 {
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
