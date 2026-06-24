use crate::tui::animation::animation::{Animation, AnimationState};
use crate::tui::animation::player::AnimationState::{HighSpend, LowSpend, MediumSpend, NoSpend};
use AnimationState::BrainRot;
use core::array::from_fn;
use ratatui::style::Stylize;
use ratatui::text::Line;

const NO_FIRE: &str = include_str!("../assets/no_fire.txt");
const LOW_FIRE: &str = include_str!("../assets/low_fire.txt");
const MEDIUM_FIRE: &str = include_str!("../assets/medium_fire.txt");
const HIGH_FIRE: &str = include_str!("../assets/high_fire.txt");
const BRAINROT: &str = include_str!("../assets/brainrot.txt");
const LEVELS: usize = 5;
const ANIMATION_LEVELS: [&str; LEVELS] = [NO_FIRE, LOW_FIRE, MEDIUM_FIRE, HIGH_FIRE, BRAINROT];

#[derive(Debug)]
pub struct AnimationPlayer {
    frame: usize,
    state: AnimationState,
    rendered_frames: [Animation; LEVELS],
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        let animations: [Animation; LEVELS] =
            from_fn(|animation_type| Animation::new(ANIMATION_LEVELS[animation_type]));
        Self {
            frame: 0,
            state: NoSpend,
            rendered_frames: animations,
        }
    }
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

    pub fn fetch_frame(&self) -> Vec<Line<'_>> {
        match self.state {
            NoSpend => &self.rendered_frames[0],
            LowSpend => &self.rendered_frames[1],
            MediumSpend => &self.rendered_frames[2],
            HighSpend => &self.rendered_frames[3],
            BrainRot => &self.rendered_frames[4],
        }
        .get_frame(self.frame)
        .content()
    }
}
