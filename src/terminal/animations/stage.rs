use crate::terminal::{clear, move_up, move_down, clear_line};
use crate::terminal::animations::image::Image;
use std::fmt::Display;
use std::io::stdin;

pub struct Stage {
    height: usize,
}

impl Stage {
    pub fn new(height: usize) -> Self {
        let stage = Self { height };
        stage.clear();
        stage
    }

    pub fn clear(&self) {
        clear(self.height + 2);
        move_up(self.height + 2);
    }

    pub fn show(&self, image: &Image) {
        assert_eq!(image.len(), self.height);
        for line in image { fprintln!("{}", line) }
        move_up(self.height);
    }

    pub fn replace(&self, image: &Image) {
        assert_eq!(image.len(), self.height);
        for line in image {
            clear_line();
            fprintln!("{}", line)
        }
        move_up(self.height);
    }

    pub fn prompt(&self, text: impl Display) {
        move_down(self.height);
        fprint!("{}", text);
        let _ = stdin().read_line(&mut String::new()).expect("failed to read line");
        clear_line();
        move_up(1);
        clear_line();
        move_up(self.height);
    }
}