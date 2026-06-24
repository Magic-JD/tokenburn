use ansi_parser::{AnsiParser, AnsiSequence, Output};
use ratatui::prelude::{Color, Line, Span, Style};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Frame {
    content: Vec<Line<'static>>,
}

impl Frame {
    pub fn new(parser: &dyn FrameParser, text: &str) -> Self {
        let content = text
            .lines()
            .map(|line| Line::from(parser.split_line(line)))
            .collect::<Vec<_>>();
        Self { content }
    }

    pub fn content(&self) -> Vec<Line<'_>> {
        self.content.clone()
    }
}

#[derive(Debug)]
pub struct GifFrame;

#[derive(Debug)]
pub struct FilmFrame;

impl FrameParser for GifFrame {
    fn split_line(&self, line: &str) -> Vec<Span<'static>> {
        let mut spans = vec![];
        for c in line.chars() {
            let span = Span::from(String::from(c));
            match c {
                ' ' => spans.push(span),
                '▒' => spans.push(span.style(Color::Yellow)),
                '▓' => spans.push(span.style(Color::Red)),
                '░' => spans.push(span.style(Color::Red)),
                _ => spans.push(span.style(Color::White)),
            }
        }
        spans
    }
}

impl FrameParser for FilmFrame {
    fn split_line(&self, line: &str) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        let mut span = Span::default();
        line.ansi_parse().for_each(|ansi| match ansi {
            Output::TextBlock(txt) => {
                if span == Span::default() {
                    span.style = Style::from(Color::Black);
                }
                span.content = Cow::from(<&str as Into<String>>::into(txt));
                spans.push(span.clone());
                span = Span::default();
            }
            Output::Escape(AnsiSequence::SetGraphicsMode(v)) => {
                if v.len() >= 5 {
                    span.style = Style::from(Color::Rgb(v[2], v[3], v[4]));
                    span.style.bg = Option::from(Color::Rgb(v[2], v[3], v[4]))
                }
            }
            _ => panic!("Unknown output type"),
        });
        spans
    }
}

pub trait FrameParser {
    fn split_line(&self, line: &str) -> Vec<Span<'static>>;
}
