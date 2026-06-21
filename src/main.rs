mod tui;
mod data;
mod calculator;
mod configuration;
mod listener;
mod utils;
mod cli;
mod actions;

use crate::cli::command::Cli;
use clap::Parser;

use crate::tui::tui::App;
use std::io;
use crate::actions::generate_config;
use crate::configuration::config::Config;

fn main() -> io::Result<()> {
    let args = Cli::parse();
    if args.task.generate_config {
        generate_config::run();
        return Ok(());
    }
    Config::init(args.config);

    ratatui::run(|terminal| App::default().run(terminal))
}


