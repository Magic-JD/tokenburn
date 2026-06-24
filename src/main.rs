mod actions;
mod calculator;
mod cli;
mod configuration;
mod data;
mod listener;
mod tui;
mod utils;

use crate::cli::command::Cli;
use clap::Parser;

use crate::actions::generate_config;
use crate::configuration::config::Config;
use crate::tui::tui::App;
use std::io;

fn main() -> io::Result<()> {
    let args = Cli::parse();
    if args.task.generate_config {
        generate_config::run();
        return Ok(());
    }
    Config::init(args.config);
    ratatui::run(|terminal| App::default().run(terminal))
}
