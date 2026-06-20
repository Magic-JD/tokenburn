use std::sync::mpsc::Sender;
use crossterm::event;
use crossterm::event::KeyCode;

pub struct KeyListener{}

impl KeyListener {

    pub fn listen(sender: Sender<bool>) {
        let mut running = true;
        while running {
            if let Some(key) = event::read().unwrap().as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        running = false;
                        sender.send(true).unwrap()
                    },
                    _ => {}
                }
            }
        }
    }
}