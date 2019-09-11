macro_rules! m {
    ($name: ident) => { |x| x.$name() }
}

extern crate rand;
use rand::{Rng, prelude::ThreadRng, seq::SliceRandom};
mod timer;
use timer::run_timer;
use crate::terminal::animations::StageWithEnterLine;

pub struct Game<'a> {
    words: &'a [&'a str],
    rng: ThreadRng,
    round: usize,
    stage: StageWithEnterLine,
}

fn scroll<S>(mut lines: Vec<S>) -> String where String: From<S> {
    let mut drain = lines.drain(..);
    format!("
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
        drain.next().map(String::from).unwrap_or(String::new()),
        drain.next().map(String::from).unwrap_or(String::new()),
        drain.next().map(String::from).unwrap_or(String::new()),
    )
}

impl<'a> Game<'a> {
    pub fn new(words: &'a [&'a str]) -> Self {
        Self {
            words,
            rng: rand::thread_rng(),
            round: 1,
            stage: StageWithEnterLine::from(vec![]),
        }
    }

    fn show_ask_hide<S: Into<String>, A: Into<String>>(&mut self, show: S, ask: A) {
        self.stage.set_string(show.into());
        self.stage.animate_up();
        self.stage.prompt(ask.into());
        self.stage.animate_down();
    }
    
    pub fn play_round(&mut self) {
        println!("\n=== ROUND {} ===\n", self.round);
        self.round += 1;
        
        let spy = (self.rng.gen::<f64>() * 3.0) as u64 + 1;
        let word = self.words.choose(&mut self.rng).unwrap();

        self.stage.fix_bottom(1);

        for player in 1..=3 {
            self.show_ask_hide(
                scroll(vec![
                    "PASS THE TERMINAL", &format!("TO PLAYER {}", player)
                ]),
                format!("PLAYER {}, PRESS ENTER", player)
            );

            self.show_ask_hide(
                scroll(if player == spy {
                    vec![".", "YOU ARE THE SPY"]
                } else {
                    vec!["THE WORD IS:", word]
                }),
                format!("PLAYER {}: PRESS ENTER WHEN YOU'RE DONE", player)
            );
        }
        
        run_timer(10);
    }
}