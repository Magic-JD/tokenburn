use crate::calculator::calculator::Calculator;
use ratatui::{
    buffer::Buffer, layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
    Frame,
};
use std::{io, thread, time};
use std::sync::mpsc;

use crate::configuration::config::Config;
use crate::listener::key_listener::KeyListener;
use crate::tui::tui::AnimationState::{HighSpend, LowSpend, NoSpend};

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let config = Config::get_config();
        let mut calculator = Calculator::new();
        let mut require_calculator = 0;
        let (sender, receiver) = mpsc::channel();
        thread::spawn( move || KeyListener::listen(sender) );
        while !receiver.try_recv().is_ok() {
            if require_calculator == 0 {
                self.cost_per_minute = calculator.current_cost_per_minute();
            }
            self.animation.set_state(self.cost_per_minute);
            terminal.draw(|frame| self.draw(frame))?;
            require_calculator += 1;
            require_calculator %= config.frames_per_second;
            thread::sleep(time::Duration::from_millis(1000/config.frames_per_second));
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
        let footer = Line::from(" Burn Baby Burn ");
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(footer.centered())
            .border_set(border::THICK);

        let info_line = Line::from(format!("Token Burn per Minute: ${:.2}", self.cost_per_minute));
        let animation = self.animation.generate_frame();
        let mut animation_lines = animation.lines().map(|line| Line::from(line)).collect::<Vec<_>>();
        animation_lines.insert(0, info_line);
        let counter_text = Text::from(animation_lines);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }

}

#[derive(Debug, Default)]
struct Animation{
    frame: i8,
    state: AnimationState,
}

#[derive(Debug, Default)]
enum AnimationState {
    #[default]
    NoSpend,
    LowSpend,
    HighSpend,
}

impl Animation {

    pub fn set_state(&mut self, spend: f32) {
        self.state = match spend {
            0.0 => NoSpend,
            spend if spend < 1.0 => LowSpend,
            _ => { HighSpend }
        };
        self.frame += 1;
        self.frame %= 2;
    }


    fn generate_frame(&self) -> String {
        match self.state {
            NoSpend => self.no_spend(),
            LowSpend => self.low_spend(),
            HighSpend => self.high_spend(),
        }
    }

    fn no_spend(&self) -> String {
        match self.frame {
            0 => {
            "


















                                                  $
                                                   $
                                                    $
                                                     $
                                                      $
                                                     $
                               .@@@@:               $           @@@@
                              @@@     @@@@         $      @@@@      @@
                              @:    @      *@@@      @@@@           -@
                              @@@              .@@@             @    @
                              @@@@@@@@    @@@@     .@   @@       @@@@@
                                  @@@@@@=     @@            @@@@@@@
                                @@@                   -@@@@@@@  @@@@
                              @ @@  @@           @@@@@@@@         @@ @
                              @   @@   :@   @@@@@@@@@@@@@@@    @@@   @
                              #@   @@@ @@@@@@@@      @@@@@@@@@@@@   @@
                                @  @@@@@@@.               @@@@@@@  @@
                                  @@@                          @@@@


       ".into()
            }
            1 => {

                "


















                                                  $
                                                   $
                                                    $
                                                   $
                                                  $
                                                   $
                               .@@@@:               $           @@@@
                              @@@     @@@@         $      @@@@      @@
                              @:    @      *@@@      @@@@           -@
                              @@@              .@@@             @    @
                              @@@@@@@@    @@@@     .@   @@       @@@@@
                                  @@@@@@=     @@            @@@@@@@
                                @@@                   -@@@@@@@  @@@@
                              @ @@  @@           @@@@@@@@         @@ @
                              @   @@   :@   @@@@@@@@@@@@@@@    @@@   @
                              #@   @@@ @@@@@@@@      @@@@@@@@@@@@   @@
                                @  @@@@@@@.               @@@@@@@  @@
                                  @@@                          @@@@

".into()
            }
            _ => panic!(),
        }

    }

    fn low_spend(&self) -> String {
        match self.frame {
            0 => frame_one(),
            1 => frame_two(),
            _ => panic!()
        }
    }

    fn high_spend(&self) -> String {
        match self.frame {
            0 => "Big spender!".into(),
            1 => "Big spender!!".into(),
            _ => panic!()
        }
    }

}

fn frame_one() -> String {
    "



                                              @   @@
                                             @@   @@#
                                            @@@*   @
                                            @@@@
                                            @@@@@    @@
                                            @@@@@@  @@@
                                       @    @@@@@@@-@@@
                                      @@@    @@@@@@@@@@@
                                       @@  @  @@@@@@@@@@@
                                           @@  @@@@@@@@@@@
                                          @@@@   @@@@@@@@@@@
                                         @@@@@    @@@@@@@@@@@@
                                       .@@@@@      @@@@@@@@@@@@@
                                      @@@@@@        @@@@@@#@@@@@@
                                     @@@@@@    @     @@@@@  @@@@@
                                    @@@@@@@   @@     @@@@@  @@@@@@
                                   @@@ @@@@  @@@     @@@@@   @@@@
                                   @@  @@@@ @@@@     @@@@@   @@@@
                                   @@   @@@@@#@@@   @@@@@    @@@
                                    @%   @@@@  @@@ @@@@@    @@@
                                     @    @@@   @@@@@@     :@@
                               .@@@@:       @@   @@@       @    @@@@
                              @@@     @@@@                @@@@      @@
                              @:    @      *@@@      @@@@           -@
                              @@@              .@@@             @    @
                              @@@@@@@@    @@@@     .@   @@       @@@@@
                                  @@@@@@=     @@            @@@@@@@
                                @@@                   -@@@@@@@  @@@@
                              @ @@  @@           @@@@@@@@         @@ @
                              @   @@   :@   @@@@@@@@@@@@@@@    @@@   @
                              #@   @@@ @@@@@@@@      @@@@@@@@@@@@   @@
                                @  @@@@@@@.               @@@@@@@  @@
                                  @@@                          @@@@


       ".into()
}

fn frame_two() -> String {
    "



                                              @
                                             @@
                                            @@@*
                                            @@@@
                                            @@@@@    @@
                                            @@@@@@  @@@
                                            @@@@@@@-@@@
                                             @@@@@@@@@@@
                                           @  @@@@@@@@@@@
                                           @@  @@@@@@@@@@@
                                          @@@@   @@@@@@@@@@@
                                         @@@@@    @@@@@@@@@@@@
                                       .@@@@@      @@@@@@@@@@@@@
                                      @@@@@@        @@@@@@#@@@@@@
                                     @@@@@@    @     @@@@@  @@@@@
                                    @@@@@@@   @@     @@@@@  @@@@@@
                                   @@@ @@@@  @@@     @@@@@   @@@@
                                   @@  @@@@ @@@@     @@@@@   @@@@
                                   @@   @@@@@#@@@   @@@@@    @@@
                                    @%   @@@@  @@@ @@@@@    @@@
                                     @    @@@   @@@@@@     :@@
                               .@@@@:       @@   @@@       @    @@@@
                              @@@     @@@@                @@@@      @@
                              @:    @      *@@@      @@@@           -@
                              @@@              .@@@             @    @
                              @@@@@@@@    @@@@     .@   @@       @@@@@
                                  @@@@@@=     @@            @@@@@@@
                                @@@                   -@@@@@@@  @@@@
                              @ @@  @@           @@@@@@@@         @@ @
                              @   @@   :@   @@@@@@@@@@@@@@@    @@@   @
                              #@   @@@ @@@@@@@@      @@@@@@@@@@@@   @@
                                @  @@@@@@@.               @@@@@@@  @@
                                  @@@                          @@@@


       ".into()
}
