use dirs::{config_dir, home_dir};
use std::path::PathBuf;
use std::{env, fs};

pub fn get_user_base_config_file() -> Option<String> {
    let buff = config_path();
    fs::read_to_string(buff).ok()
}
pub fn config_path() -> PathBuf {
    let mut path = config_location();
    path.push("config.toml");
    path
}

pub fn claude_project_path() -> PathBuf {
    let claude_project_dir = home_dir()
        .map(|mut path| {
            path.push(".claude/projects/");
            path
        })
        .unwrap();
    claude_project_dir
}

fn config_location() -> PathBuf {
    env_default_path("TOKENBURN_CONFIG_DIR", config_dir)
}

fn env_default_path(env_var_name: &str, default: fn() -> Option<PathBuf>) -> PathBuf {
    env::var(env_var_name)
        .map(PathBuf::from)
        .ok()
        .or_else(|| fs_default_path(default))
        .expect("Unable to determine the path.")
}

fn fs_default_path(default: fn() -> Option<PathBuf>) -> Option<PathBuf> {
    default()
        .map(|mut path| {
            path.push("tokenburn");
            path
        })
        .inspect(|path| fs::create_dir_all(path).expect("Failed to create directory"))
}
