use dirs::home_dir;
use rev_lines::RevLines;
use serde::{Deserialize, Serialize};
use serde_json;
use sha_file_hashing::Hashable;
use std::collections::{HashMap, HashSet};
use std::fs::DirEntry;
use std::fs::File;
use std::iter::{Filter, FilterMap};
use std::path::PathBuf;
use std::{io, thread, time};
use std::ffi::OsString;
use walkdir::{IntoIter, WalkDir};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
    // Challenges:
    // Smoothing. Idea, each seconds token use is spread over 10s following a standard distribution. We add the previous seconds distribution to any new ones. This will give a smooth rise and fall. Configurable? -> work out a better algorithm for this.
    // Have to learn how to make ascii art.
}

fn log_files() -> Vec<DirEntry> {
    WalkDir::new(claude_project_dir())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_dir())
        .flat_map(|e| std::fs::read_dir(e.into_path()).unwrap())
        .filter_map(|e| e.ok())
        .filter(|e| e.metadata().unwrap().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .filter(|str| str.to_string_lossy() == "jsonl")
                .is_some()
        })
        .collect::<Vec<_>>()
}

fn claude_project_dir() -> PathBuf {
    home_dir()
        .map(|mut path| {
            path.push(".claude/projects/");
            path
        })
        .unwrap()
}

struct Cost {
    input: isize,
    output: isize,
}

impl Cost {
    fn new(input: isize, output: isize) -> Cost {
        Cost { input, output }
    }
}

#[derive(Deserialize, Debug)]
struct TokenLog {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Message {
    id: String,
    model: String,
    usage: Usage,
}

#[derive(Deserialize, Debug, Serialize)]
struct Usage {
    input_tokens: isize,
    output_tokens: isize,
}

impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut token_smoothing = [0f32; 10];

        // Get token data from online somewhere?
        let costs = HashMap::from([
            ("<synthetic>", Cost::new(10, 100)),
            ("qwen3.5:0.8b", Cost::new(1, 10)),
            ("qwen2.5-coder:0.5b", Cost::new(10, 100)),
        ]);

        let mut file_to_hash = HashMap::new();
        let mut seen_logs: HashSet<String> = HashSet::new();
        Self::ignore_seen_logs(&mut file_to_hash, &mut seen_logs);
        loop {
            let cost_this_sec = Self::calc_cost_per_sec(&costs, &mut file_to_hash, &mut seen_logs);
            let mut cost_per_minute = (cost_this_sec * 60f32);

            if cost_per_minute != 0f32 {
                cost_per_minute /= 2f32;
                let division_factor = 2f32;
                let center = token_smoothing.len() / 2;
                for i in 0..center - 1 {
                    cost_per_minute /= division_factor;
                    token_smoothing[center - i - 1] += cost_per_minute;
                    token_smoothing[center + i] += cost_per_minute;
                }
                token_smoothing[0] += cost_per_minute;
                token_smoothing[token_smoothing.len() - 1] += cost_per_minute;
            }

            for i in 0..token_smoothing.len() - 1 {
                token_smoothing[i] = token_smoothing[i + 1];
            }
            token_smoothing[token_smoothing.len() - 1] = 0f32;
            self.cost_per_minute = token_smoothing[0];
            terminal.draw(|frame| self.draw(frame))?;
            thread::sleep(time::Duration::from_secs(1));
        }
        Ok(())
    }

    fn calc_cost_per_sec(costs: &HashMap<&str, Cost>, file_to_hash: &mut HashMap<OsString, String>, seen_logs: &mut HashSet<String>) -> f32 {
        let mut cost_this_sec = 0f32;
        for file in log_files() {
            if file_to_hash
                .get(&file.file_name().to_owned())
                .filter(|hash| hash.to_string() == file.path().hash().unwrap())
                .is_none()
            {
                file_to_hash.insert(file.file_name().to_owned(), file.path().hash().unwrap());
                let lines = RevLines::new(File::open(file.path()).unwrap());
                for line in lines {
                    if let Ok(line) = line {
                        let token_log: Result<TokenLog, serde_json::Error> =
                            serde_json::from_str(&line);
                        if let Ok(token_log) = token_log {
                            if seen_logs.contains(&token_log.message.id) {
                                break;
                            }
                            seen_logs.insert(token_log.message.id);
                            let cost = costs.get(token_log.message.model.as_str()).expect(
                                format!("No cost for {:?}", token_log.message.model).as_str(),
                            );
                            let usage = token_log.message.usage;
                            cost_this_sec += (usage.output_tokens * cost.output
                                + usage.input_tokens * cost.input)
                                as f32
                                / 1_000_000f32;
                        }
                    }
                }
            }
        }
        cost_this_sec
    }

    fn ignore_seen_logs(file_to_hash: &mut HashMap<OsString, String>, seen_logs: &mut HashSet<String>) {
        for file in log_files() {
            file_to_hash.insert(file.file_name().to_owned(), file.path().hash().unwrap());
            let lines = RevLines::new(File::open(file.path()).unwrap());
            for line in lines {
                if let Ok(line) = line {
                    let token_log: Result<TokenLog, serde_json::Error> = serde_json::from_str(&line);
                    if let Ok(token_log) = token_log {
                        seen_logs.insert(token_log.message.id);
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

#[derive(Debug, Default)]
pub struct App {
    cost_per_minute: f32,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Token Burn ".bold());
        let footer = Line::from(" Burn baby Burn ");
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(footer.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Cost per minute: ".into(),
            self.cost_per_minute.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}