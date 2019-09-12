extern crate rand;
use rand::{Rng, prelude::ThreadRng, seq::SliceRandom};
mod timer;
use timer::run_timer;
use crate::terminal::animations::image::Image;
use crate::terminal::animations::action_sequence::ActionSequence;
use crate::terminal::animations::stage::Stage;

pub struct Game<'a> {
    words: &'a [&'a str],
    rng: ThreadRng,
    round: usize,
    stage: Stage,
}

fn small_scroll(lines: impl Into<String>) -> Image {
    let lines = lines.into();
    let mut drain = lines.split("\n");
    format!(r"
  _______________________
=(__    ___      __     _)=
  |                   . |
  |       .             |
  |{:^21}|
  |{:^21}|
  |{:^21}|
  |               .     |
  |    .                |
  |         .           |
  |       .          .  |
  |__    ___   __    ___|
=(_______________________)=",
        drain.next().unwrap_or(""),
        drain.next().unwrap_or(""),
        drain.next().unwrap_or(""),
    ).into()
}

fn big_scroll(lines: impl Into<String>) -> Image {
    let lines = lines.into();
    let mut drain = lines.split("\n");
    format!(r"
 .-.---------------------------------.-.
((o))__ _   ___ __  __._____ _   _ ___  )
 \U/_______    .     _____         _.__/
   |         .    .                   |
   |    .                         .   |
   |             .          .         |
   |{:^34}|
   |{:^34}|
   |{:^34}|
   |               .           .      |
   |  .        .                      |
   |                                  |
   |       .          .        .      |
   |        .                         |
   |____    _______    __  ____    ___|
  /A\     .        .               .   \
 ((o))_ __  ___ _____ _.  __ ___ ____ __)
  '-'----------------------------------'",
            drain.next().unwrap_or(""),
            drain.next().unwrap_or(""),
            drain.next().unwrap_or(""),
    ).into()
}

impl<'a> Game<'a> {
    pub fn new(words: &'a [&'a str]) -> Self {
        Self {
            words,
            rng: rand::thread_rng(),
            round: 1,
            stage: Stage::new(big_scroll("").len()), // check len of a big scroll
        }
    }

    fn show_ask_hide<S: Into<String>>(&mut self, fixed_bottom: usize, show: &Image, ask: S) {
        ActionSequence::build_with(show)
            .fix_bottom(fixed_bottom)
            .in_from_below()
            .prompt(ask.into())
            .out_to_below()
            .build()
            .execute(&self.stage)
    }
    
    pub fn play_round(&mut self) {
        println!("\n=== ROUND {} ===\n", self.round);
        self.round += 1;
        
        let spy = (self.rng.gen::<f64>() * 3.0) as u64 + 1;
        let word = self.words.choose(&mut self.rng).unwrap();

//        self.stage.fix_bottom(3);

        for player in 1..=3 {
            self.show_ask_hide(
                3,
                &big_scroll(format!("\
                    PASS THE TERMINAL\n\
                    TO PLAYER {}",
                    player
                )),
                format!("PLAYER {}, PRESS ENTER", player)
            );

            self.show_ask_hide(
                3,
                &big_scroll(if player == spy {
                    ".\nYOU ARE THE SPY".to_string()
                } else {
                    format!("THE WORD IS:\n{}", word)
                }),
                format!("PLAYER {}: PRESS ENTER WHEN YOU'RE DONE", player)
            );
        }
        
        run_timer(10);
    }
}