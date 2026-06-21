use crate::data::claude_log::{Message, TokenLog};
use dirs::home_dir;
use rev_lines::RevLines;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct LogTracker {
    seen_logs: HashSet<String>,
    claude_project_dir: PathBuf,
}

impl LogTracker {
    pub fn new() -> Self {
        let mut seen_logs = HashSet::new();
        let claude_project_dir = home_dir()
            .map(|mut path| {
                path.push(".claude/projects/");
                path
            })
            .unwrap();
        for file in log_files(claude_project_dir.clone()).into_iter().map(|path| File::open(path).unwrap()) {
            for line in RevLines::new(file) {
                if let Ok(line) = line {
                    let token_log: Result<TokenLog, serde_json::Error> = serde_json::from_str(&line);
                    if let Ok(token_log) = token_log {
                        seen_logs.insert(token_log.message.id);
                        break;
                    }
                }
            }
        }
        Self { seen_logs, claude_project_dir }
    }

    pub fn new_logs(&mut self) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        for file in log_files(self.claude_project_dir.clone()).into_iter().map(|path| File::open(path).unwrap()) {
            for line in RevLines::new(file) {
                if let Ok(line) = line {
                    let token_log: Result<TokenLog, serde_json::Error> =
                        serde_json::from_str(&line);
                    if let Ok(token_log) = token_log {
                        if self.seen_logs.contains(&token_log.message.id) {
                            break;
                        }
                        self.seen_logs.insert(token_log.message.id.to_string());
                        messages.push(token_log.message);
                    }
                }
            }
        }
        messages
    }
}

fn log_files(claude_project_dir: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(claude_project_dir)
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
        .map(|e| e.path())
        .collect::<Vec<_>>()
}
