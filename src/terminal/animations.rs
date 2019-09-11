use super::{clear, save_cursor, restore_cursor};
use crate::terminal::{move_up, move_down, temporary_text, clear_line};
use std::thread::sleep;
use std::time::Duration;
use std::marker::PhantomData;
use std::io::stdin;
use std::cmp::max;

pub struct Stage(Vec<String>);

impl From<Vec<String>> for Stage {
    fn from(vec: Vec<String>) -> Self {
        Self(vec)
    }
}

impl Stage {
    pub fn new(height: usize, string: String) -> Self {
        let stage = Self(string.lines().map(String::from).collect());
        stage.clear();
        stage
    }

    pub fn set_vec(&mut self, vec: Vec<String>) {
        self.0 = vec;
        self.clear();
    }

    pub fn set_string(&mut self, new_string: String) {
        self.0 = new_string.lines().map(String::from).collect();
        self.clear();
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&self) {
        clear(self.len());
        move_up(self.len());
    }

    pub fn show(&self) {
        for line in &self.0 {
            fprintln!("{}", line)
        }
        move_up(self.len());
    }

    fn print_lines_from_bottom(&self, num_lines: usize) {
        for _ in 0..(self.len() - num_lines) { move_down(1) }
        self.0.iter().take(num_lines).for_each(|l| fprintln!("{}", l));
        move_up(self.len());
    }

    pub fn animate_up(&self) {
        for num_lines in 0..=self.len() {
            self.clear();
            self.print_lines_from_bottom(num_lines);
            sleep(Duration::from_millis(50));
        }
    }

    pub fn animate_down(&self) {
        for num_lines in (0..=self.len()).rev() {
            self.clear();
            self.print_lines_from_bottom(num_lines);
            sleep(Duration::from_millis(50));
        }
    }
}

// a formality to ensure ::new is used
pub struct EnterLine(PhantomData<()>);

fn down_and_up<R>(lines: usize, f: impl FnOnce() -> R) -> R {
    move_down(lines);
    let result = f();
    move_up(lines);
    result
}

impl EnterLine {
    pub fn new() -> Self {
        let it = EnterLine(PhantomData);
        it.clear();
        it
    }

    pub fn clear(&self) {
        clear(2);
        move_up(2);
    }

    pub fn len(&self) -> usize {
        2
    }

    pub fn prompt<S: Into<String>>(&self, s: S) {
        fprint!("{}", s.into());
        let mut _in = String::new();
        stdin().read_line(&mut _in).expect("couldn't read line");
        move_up(1);
        clear_line();
    }
}

pub struct StageWithEnterLine(Stage, EnterLine);

impl From<Vec<String>> for StageWithEnterLine {
    fn from(vec: Vec<String>) -> Self {
        let stage = Stage::from(vec);
        let enter_line = down_and_up(stage.len(), || EnterLine::new());
        Self(stage, enter_line)
    }
}

macro_rules! forward {
    ($( $st: ty, $to: tt, $name: ident ( $( $arg: ident : $type: ty ),* ) $( -> $ret: ty )?; )*) => {
        $(
            pub fn $name(self: $st, $( $arg : $type ),* ) $( -> $ret )? {
                self.$to.$name( $( $arg ),* )
            }
        )*
    };
}

impl StageWithEnterLine {
    forward! {
        &mut Self, 0, set_vec(v: Vec<String>);
        &mut Self, 0, set_string(s: String);
        &Self, 0, show();
        &Self, 0, animate_up();
        &Self, 0, animate_down();
    }

    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    pub fn clear(&self) {
        self.0.clear();
        down_and_up(self.0.len(), || self.1.clear());
    }

    pub fn prompt<S: Into<String>>(&self, s: S) {
        down_and_up(self.0.len(), || self.1.prompt(s));
    }
}