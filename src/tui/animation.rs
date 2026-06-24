use crate::tui::animation::AnimationState::{HighSpend, LowSpend, MediumSpend, NoSpend};
use AnimationState::BrainRot;
use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};
use core::array::from_fn;
use ratatui::prelude::{Color, Span};
use ratatui::style::{Style, Stylize};
use ratatui::text::Line;
use std::borrow::Cow;

const NO_FIRE: &str = include_str!("assets/no_fire.txt");
const LOW_FIRE: &str = include_str!("assets/low_fire.txt");
const MEDIUM_FIRE: &str = include_str!("assets/medium_fire.txt");
const HIGH_FIRE: &str = include_str!("assets/high_fire.txt");
const BRAINROT: &str = include_str!("assets/brainrot.txt");
const ANIMATION_HEIGHT: usize = 40;
const LEVELS: usize = 5;
const ANIMATION_LEVELS: [&str; LEVELS] = [NO_FIRE, LOW_FIRE, MEDIUM_FIRE, HIGH_FIRE, BRAINROT];

#[derive(Debug)]
pub struct AnimationPlayer {
    frame: usize,
    state: AnimationState,
    rendered_frames: [Animation; LEVELS],
}

#[derive(Debug)]
pub struct Animation {
    max_frame: usize,
    frames: Vec<Frame>,
}

#[derive(Debug)]
pub struct Frame {
    content: Vec<Line<'static>>,
}

#[derive(Debug)]
pub struct GifFrame {}

#[derive(Debug)]
pub struct FilmFrame {}

impl FrameParser for GifFrame {
    fn split_line(&self, line: String) -> Vec<Span<'static>> {
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
    fn split_line(&self, line: String) -> Vec<Span<'static>> {
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
    fn split_line(&self, line: String) -> Vec<Span<'static>>;
}

#[derive(Debug)]
enum FrameType {
    Film,
    Gif,
}

impl FrameType {
    pub(crate) fn clone(&self) -> FrameType {
        match self {
            FrameType::Film => FrameType::Film,
            FrameType::Gif => FrameType::Gif,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    NoSpend,
    LowSpend,
    MediumSpend,
    HighSpend,
    BrainRot,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        let animations: [Animation; LEVELS] =
            from_fn(|animation_type| create_animation(ANIMATION_LEVELS[animation_type]));
        Self {
            frame: 0,
            state: NoSpend,
            rendered_frames: animations,
        }
    }
}

fn create_animation(animation_file: &str) -> Animation {
    let max_frame = animation_file.lines().count() / ANIMATION_HEIGHT;
    let frame_parser: FrameType = if max_frame == 6 {
        FrameType::Gif
    } else {
        FrameType::Film
    };
    let frames = (0..max_frame)
        .map(|frame| AnimationPlayer::create_frame(frame, animation_file, frame_parser.clone()))
        .into_iter()
        .collect();
    Animation { max_frame, frames }
}

impl AnimationPlayer {
    pub fn tagline(&self) -> Line<'_> {
        let tagline: Line = match self.state {
            NoSpend => " Feed Me Money! ".into(),
            LowSpend => " Less thinking more prompting ".into(),
            MediumSpend => " Huang expects more out of you young vibe coder ".into(),
            HighSpend => " Is this more than your salary yet? ".into(),
            BrainRot => " Maximum Rot ".into(),
        };
        tagline.bold()
    }

    pub fn set_state(&mut self, spend: f32) {
        self.state = match spend {
            0.0 => NoSpend,
            spend if spend < 0.5 => LowSpend,
            spend if spend < 1.0 => MediumSpend,
            spend if spend < 2.0 => HighSpend,
            _ => BrainRot,
        };
        self.frame += 1;
    }

    pub fn fetch_frame(&self) -> Vec<Line<'static>> {
        let animation = match self.state {
            NoSpend => &self.rendered_frames[0],
            LowSpend => &self.rendered_frames[1],
            MediumSpend => &self.rendered_frames[2],
            HighSpend => &self.rendered_frames[3],
            BrainRot => &self.rendered_frames[4],
        };
        animation.frames[self.frame % animation.max_frame]
            .content
            .clone()
    }

    fn create_frame(current_frame: usize, animation_file: &str, frame_type: FrameType) -> Frame {
        let start_idx: usize = current_frame * ANIMATION_HEIGHT;
        let animation = animation_file.lines().collect::<Vec<_>>()
            [start_idx..start_idx + ANIMATION_HEIGHT]
            .join("\n");
        let content = animation
            .lines()
            .map(|line| {
                Line::from(match frame_type {
                    FrameType::Film => FilmFrame {}.split_line(String::from(line)),
                    FrameType::Gif => GifFrame {}.split_line(String::from(line)),
                })
            })
            .collect::<Vec<_>>();
        Frame { content }
    }
}
