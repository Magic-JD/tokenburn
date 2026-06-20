use crate::calculator::calculator::Calculator;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{io, thread, time};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut calculator = Calculator::new();
        loop {
            self.cost_per_minute = calculator.current_cost_per_minute();
            terminal.draw(|frame| self.draw(frame))?;
            thread::sleep(time::Duration::from_secs(1));
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