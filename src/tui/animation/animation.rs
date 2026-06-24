use crate::tui::animation::frame::{FilmFrame, Frame, FrameParser, GifFrame};

const ANIMATION_HEIGHT: usize = 40;

#[derive(Debug)]
pub struct Animation {
    max_frame: usize,
    frames: Vec<Frame>,
}

impl Animation {
    pub fn new(animation_file: &str) -> Self {
        let max_frame = animation_file.lines().count() / ANIMATION_HEIGHT;
        let frame_parser: Box<dyn FrameParser> = if max_frame == 6 {
            Box::new(GifFrame)
        } else {
            Box::new(FilmFrame)
        };
        let frames = (0..max_frame)
            .map(|frame| Self::create_frame(frame, animation_file, frame_parser.as_ref()))
            .into_iter()
            .collect();
        Self { max_frame, frames }
    }

    fn create_frame(
        current_frame: usize,
        animation_file: &str,
        frame_type: &dyn FrameParser,
    ) -> Frame {
        let start_idx: usize = current_frame * ANIMATION_HEIGHT;
        let animation = animation_file.lines().collect::<Vec<_>>()
            [start_idx..start_idx + ANIMATION_HEIGHT]
            .join("\n");
        Frame::new(frame_type, animation.as_ref())
    }

    pub fn get_frame(&self, current_frame: usize) -> &Frame {
        &self.frames[current_frame % self.max_frame]
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
