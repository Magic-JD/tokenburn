use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc::Sender;

pub struct KeyListener {}

impl KeyListener {
    pub fn listen(sender: Sender<bool>) {
        let mut running = true;
        while running {
            if let Some(key_event) = event::read().unwrap().as_key_press_event() {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Char('q') | KeyCode::Esc,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        running = false;
                        sender.send(true).unwrap()
                    }
                    _ => {}
                }
            }
        }
    }
}
