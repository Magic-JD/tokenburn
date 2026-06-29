use crate::calculator::calculator::Calculator;
use crate::configuration::config::Config;
use crate::listener::key_listener::KeyListener;
use crate::tui::animation::player::AnimationPlayer;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::sync::mpsc;
use std::{io, thread, time};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let config = Config::get_config();
        let max_sleep = 1000 / config.frames_per_second as u64;
        let mut calculator = Calculator::new();
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || KeyListener::listen(sender));
        while !receiver.try_recv().is_ok() {
            let start = time::SystemTime::now();
            self.cost_per = calculator.current_cost_per_minute();
            self.animation.set_state(self.cost_per);
            terminal.draw(|frame| self.draw(frame))?;
            let time_passed = start.elapsed().unwrap().as_millis() as u64;
            if time_passed < max_sleep {
                thread::sleep(time::Duration::from_millis(max_sleep - time_passed));
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

#[derive(Debug, Default)]
pub struct App {
    cost_per: f32,
    animation: AnimationPlayer,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Token Burn ".bold());
        let footer = self.animation.tagline();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(footer.centered())
            .border_set(border::THICK);
        let per_x_seconds = Config::get_config().per_x_seconds;
        let info_line: Line = Line::from(format!(
            "Token Burn per {}: ${:.2}",
            format_duration(per_x_seconds),
            self.cost_per
        ));
        let mut animation_lines = self.animation.fetch_frame();
        let len = animation_lines.len();
        let height = area.height as usize - 3;
        if len < height {
            let shortfall = height - len;
            for _ in 0..shortfall {
                animation_lines.insert(0, Line::default());
            }
        } else if len > height {
            animation_lines =
                animation_lines[animation_lines.len() - height..animation_lines.len()].to_owned();
        }
        animation_lines.insert(0, info_line);
        let counter_text = Text::from(animation_lines);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
fn format_duration(duration: u32) -> String {
    match duration {
        1 => "Second".to_string(),
        60 => "Minute".to_string(),
        3600 => "Hour".to_string(),
        _ => {
            let hours = duration / 60 / 60;
            let minutes = duration / 60 % 60;
            let seconds = duration % 60 % 60;
            let mut parts = Vec::new();
            if hours > 0 {
                parts.push(format!(
                    "{} Hour{}",
                    hours,
                    if hours > 1 { "s" } else { "" }
                ));
            }
            if minutes > 0 {
                parts.push(format!(
                    "{} Minute{}",
                    minutes,
                    if minutes > 1 { "s" } else { "" }
                ));
            }
            if seconds > 0 {
                parts.push(format!(
                    "{} Second{}",
                    seconds,
                    if seconds > 1 { "s" } else { "" }
                ));
            }
            parts.join(" ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_test() {
        assert_eq!(format_duration(1), "Second".to_string());
        assert_eq!(format_duration(2), "2 Seconds".to_string());
        assert_eq!(format_duration(60), "Minute".to_string());
        assert_eq!(format_duration(61), "1 Minute 1 Second".to_string());
        assert_eq!(format_duration(62), "1 Minute 2 Seconds".to_string());
        assert_eq!(format_duration(122), "2 Minutes 2 Seconds".to_string());
        assert_eq!(format_duration(3600), "Hour".to_string());
        assert_eq!(format_duration(3601), "1 Hour 1 Second".to_string());
        assert_eq!(format_duration(3602), "1 Hour 2 Seconds".to_string());
        assert_eq!(format_duration(3660), "1 Hour 1 Minute".to_string());
        assert_eq!(
            format_duration(3661),
            "1 Hour 1 Minute 1 Second".to_string()
        );
        assert_eq!(
            format_duration(3662),
            "1 Hour 1 Minute 2 Seconds".to_string()
        );
        assert_eq!(format_duration(3720), "1 Hour 2 Minutes".to_string());
        assert_eq!(
            format_duration(3721),
            "1 Hour 2 Minutes 1 Second".to_string()
        );
        assert_eq!(
            format_duration(3722),
            "1 Hour 2 Minutes 2 Seconds".to_string()
        );
        assert_eq!(format_duration(7200), "2 Hours".to_string());
        assert_eq!(format_duration(7201), "2 Hours 1 Second".to_string());
        assert_eq!(format_duration(7202), "2 Hours 2 Seconds".to_string());
        assert_eq!(format_duration(7260), "2 Hours 1 Minute".to_string());
        assert_eq!(
            format_duration(7261),
            "2 Hours 1 Minute 1 Second".to_string()
        );
        assert_eq!(
            format_duration(7262),
            "2 Hours 1 Minute 2 Seconds".to_string()
        );
        assert_eq!(format_duration(7320), "2 Hours 2 Minutes".to_string());
        assert_eq!(
            format_duration(7321),
            "2 Hours 2 Minutes 1 Second".to_string()
        );
        assert_eq!(
            format_duration(7322),
            "2 Hours 2 Minutes 2 Seconds".to_string()
        );
    }
}
