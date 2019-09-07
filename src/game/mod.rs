extern crate rand;
use rand::{Rng, prelude::ThreadRng, seq::SliceRandom};
mod timer;
use timer::run_timer;
use crate::terminal::{pre_clear, show_secret_text};

pub struct Game<'a> {
    words: &'a [&'a str],
    rng: ThreadRng,
    round: usize,
}

impl<'a> Game<'a> {
    pub fn new(words: &'a [&'a str]) -> Self {
        Self {
            words,
            rng: rand::thread_rng(),
            round: 1,
        }
    }
    
    pub fn play_round(&mut self) {
        println!("\n=== ROUND {} ===\n", self.round);
        self.round += 1;
        
        let spy = self.rng.gen_range(0, 2);
        let word = self.words.choose(&mut self.rng).unwrap();
        
        pre_clear(2);
        
        for player in 0..3 {
            show_secret_text(
                format!("Pass the terminal to player {}", player + 1),
                if player == spy {
                    "You are the spy".to_string()
                } else {
                    format!("The word is: {}", word)
                },
                "PRESS ENTER WHEN DONE READING".to_string()
            );
        }
        
        run_timer(10);
    }
}