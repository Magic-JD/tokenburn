use crate::calculator::calculator::Calculator;
use crate::configuration::config::Config;
use crate::listener::key_listener::KeyListener;
use crate::tui::animation::Animation;
use ratatui::{
    buffer::Buffer, layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
    Frame,
};
use std::sync::mpsc;
use std::{io, thread, time};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let config = Config::get_config();
        let mut calculator = Calculator::new();
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || KeyListener::listen(sender));
        while !receiver.try_recv().is_ok() {
            let start = time::SystemTime::now();
            self.cost_per_minute = 0.0f32;//calculator.current_cost_per_minute();
            self.animation.set_state(self.cost_per_minute);
            terminal.draw(|frame| self.draw(frame))?;
            let duration = start.elapsed().unwrap();
            thread::sleep(time::Duration::from_millis((1000 / config.frames_per_second as u64) - duration.as_millis() as u64));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

#[derive(Debug, Default)]
pub struct App {
    cost_per_minute: f32,
    animation: Animation,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Token Burn ".bold());
        let footer = self.animation.tagline();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(footer.centered())
            .border_set(border::THICK);
        let info_line = Line::from(format!("Token Burn per Minute: ${:.2}", self.cost_per_minute));
        let mut animation_lines = self.animation.fetch_frame();
        let len = animation_lines.len();
        let height = area.height as usize - 3;
        if len < height {
            let shortfall = height - len;
            for _ in 0..shortfall {
                animation_lines.insert(1, Line::default());
            }
        } else if len > height {
            animation_lines = animation_lines[animation_lines.len() - height..animation_lines.len()].to_owned();
        }
        animation_lines.insert(0, info_line);
        let counter_text = Text::from(animation_lines);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

