mod tui;
mod data;
mod calculator;

use crate::tui::tui::App;
use std::io;

fn main() -> io::Result<()> {
    // Challenges:
    // Smoothing. Idea, each seconds token use is spread over 10s following a standard distribution. We add the previous seconds distribution to any new ones. This will give a smooth rise and fall. Configurable? -> work out a better algorithm for this.
    // Have to learn how to make ascii art.
    ratatui::run(|terminal| App::default().run(terminal))
}


