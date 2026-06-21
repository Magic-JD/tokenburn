use crate::tui::animation::AnimationState::{HighSpend, LowSpend, MediumSpend, NoSpend};
use core::array::from_fn;
use ratatui::prelude::{Color, Span};
use ratatui::style::Stylize;
use ratatui::text::Line;

const NO_FIRE: &str = include_str!("assets/no_fire.txt");
const LOW_FIRE: &str = include_str!("assets/low_fire.txt");
const MEDIUM_FIRE: &str = include_str!("assets/medium_fire.txt");
const HIGH_FIRE: &str = include_str!("assets/high_fire.txt");
const FRAME_COUNT: usize = 6;
const ANIMATION_HEIGHT: usize = 40;
const ANIMATION_LEVELS: [&str; 4] = [NO_FIRE, LOW_FIRE, MEDIUM_FIRE, HIGH_FIRE];

#[derive(Debug)]
pub struct Animation {
    frame: usize,
    state: AnimationState,
    rendered_frames: [[Vec<Line<'static>>; FRAME_COUNT]; ANIMATION_LEVELS.len()],
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    NoSpend,
    LowSpend,
    MediumSpend,
    HighSpend,
}

impl Default for Animation {
    fn default() -> Self {
        let animations: [[Vec<Line>; 6]; 4] = from_fn(|animation_type| {
            from_fn(|current_frame| {
                Self::create_frame(animation_type, current_frame)
            })
        });
        Self {
            frame: 0,
            state: NoSpend,
            rendered_frames: animations,
        }
    }
}

impl Animation {
    pub fn tagline(&self) -> Line<'_> {
        let tagline: Line = match self.state {
            NoSpend => " Feed Me Money! ".into(),
            LowSpend => " Less thinking more prompting ".into(),
            MediumSpend => " Huang expects more out of you young vibe coder ".into(),
            HighSpend => " Is this more than your salary yet? ".into(),
        };
        tagline.bold()
    }

    pub fn set_state(&mut self, spend: f32) {
        self.state = match spend {
            0.0 => NoSpend,
            spend if spend < 0.5 => LowSpend,
            spend if spend < 1.0 => MediumSpend,
            _ => HighSpend,
        };
        self.frame += 1;
        self.frame %= FRAME_COUNT;
    }

    pub fn fetch_frame(&self) -> Vec<Line<'static>> {
        match self.state {
            NoSpend => self.rendered_frames[0][self.frame].clone(),
            LowSpend => self.rendered_frames[1][self.frame].clone(),
            MediumSpend => self.rendered_frames[2][self.frame].clone(),
            HighSpend => self.rendered_frames[3][self.frame].clone(),
        }
    }

    fn split_line(line: String) -> Vec<Span<'static>> {
        let mut spans = vec![];
        for c in line.chars() {
            match c {
                ' ' => spans.push(Span::from(" ")),
                '▒' => spans.push(Span::from("▒").style(Color::Yellow)),
                '▓' => spans.push(Span::from("▓").style(Color::Red)),
                '░' => spans.push(Span::from("░").style(Color::Red)),
                _ => panic!(
                    "{}",
                    format!("Invalid character '{:?}' in line '{:?}'", c, line)
                ),
            }
        }
        spans
    }

    fn create_frame(animation_type: usize, current_frame: usize) -> Vec<Line<'static>> {
        let start_idx: usize = current_frame * ANIMATION_HEIGHT;
        let animation = ANIMATION_LEVELS[animation_type].lines().collect::<Vec<_>>()
            [start_idx..start_idx + ANIMATION_HEIGHT]
            .join("\n");
        animation
            .lines()
            .map(|line| Line::from(Animation::split_line(line.into())))
            .collect::<Vec<_>>()
    }
}
