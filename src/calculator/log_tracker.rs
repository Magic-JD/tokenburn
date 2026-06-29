use crate::configuration::config::Config;
use crate::data::claude_log::{Message, TokenLog};
use crate::utils::file_system::claude_project_path;
use rev_lines::RevLines;
use std::collections::{HashMap, HashSet};
use std::fs::{read_dir, DirEntry as FsDirEntry, File};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct LogTracker {
    seen_logs: HashSet<String>,
    file_checklist: HashMap<PathBuf, SystemTime>,
    claude_project_dir: PathBuf,
    new_files_check: SystemTime,
}

impl LogTracker {
    pub fn new() -> Self {
        let fps = Config::get_config().frames_per_second;
        let base_time = SystemTime::now() + Duration::from_secs(1);
        let claude_project_dir = claude_project_path();

        let files_to_check = Self::all_files(claude_project_dir.clone());

        let file_checklist = files_to_check
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let additional =
                    Duration::from_millis(((1000 / fps as usize) * (i % fps as usize)) as u64);
                (path.clone(), base_time + additional)
            })
            .collect();

        let seen_logs = files_to_check
            .into_iter()
            .map(|path| File::open(path).unwrap())
            .filter_map(|file| {
                RevLines::new(file)
                    .flatten()
                    .filter_map(|line| serde_json::from_str::<TokenLog>(&line).ok())
                    .next()
            })
            .map(|log| log.message.id)
            .collect();

        Self {
            seen_logs,
            file_checklist,
            claude_project_dir,
            new_files_check: base_time,
        }
    }

    pub fn new_logs(&mut self) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();
        let current_time = SystemTime::now();
        let files_needing_checking: Vec<PathBuf> = self
            .log_files(self.claude_project_dir.clone())
            .into_iter()
            .filter(|file| self.needs_checking(file, current_time))
            .collect();
        for file in files_needing_checking
            .into_iter()
            .filter_map(|path| File::open(path).ok())
        {
            for token_log in RevLines::new(file)
                .flatten()
                .filter_map(|line| serde_json::from_str::<TokenLog>(&line).ok())
            {
                if self.seen_logs.contains(&token_log.message.id) {
                    break;
                }
                self.seen_logs.insert(token_log.message.id.to_string());
                messages.push(token_log.message);
            }
        }
        messages
    }

    fn needs_checking(&mut self, path: &PathBuf, current_time: SystemTime) -> bool {
        let time = self.file_checklist.get(path);
        let result = match time {
            Some(t) => current_time.duration_since(*t).is_ok(),
            None => true,
        };
        if result {
            self.file_checklist.insert(
                path.clone(),
                *time.unwrap_or(&current_time) + Duration::from_secs(1),
            );
        }
        result
    }

    fn log_files(&mut self, claude_project_dir: PathBuf) -> Vec<PathBuf> {
        if !SystemTime::now()
            .duration_since(self.new_files_check)
            .is_ok()
        {
            return self.file_checklist.keys().cloned().collect();
        }
        self.new_files_check = SystemTime::now()
            + Duration::from_secs(1)
            + Duration::from_millis(1000 / Config::get_config().frames_per_second as u64);
        Self::all_files(claude_project_dir)
    }

    fn all_files(claude_project_dir: PathBuf) -> Vec<PathBuf> {
        Self::jsonl_recursive_search(
            read_dir(claude_project_dir)
                .unwrap()
                .flatten()
                .collect(),
        )
    }

    fn jsonl_recursive_search(entry: Vec<FsDirEntry>) -> Vec<PathBuf> {
        entry
            .into_iter()
            .flat_map(|entry| {
                if entry.file_type().unwrap().is_dir() {
                    let dir = read_dir(entry.path())
                        .unwrap()
                        .flatten()
                        .collect();
                    Self::jsonl_recursive_search(dir)
                } else {
                    match entry.path().extension().and_then(|ext| ext.to_str()) {
                        Some("jsonl") => vec![entry.path()],
                        _ => vec![],
                    }
                }
            })
            .collect()
    }
}
