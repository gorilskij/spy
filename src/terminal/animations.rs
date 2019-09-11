use super::{clear, save_cursor, restore_cursor};
use crate::terminal::{move_up, move_down, temporary_text, clear_line};
use std::thread::sleep;
use std::time::Duration;
use std::marker::PhantomData;
use std::io::{stdin, Read};
use std::cmp::max;
use std::fmt::Display;

pub struct Stage {
    vec: Vec<String>,
    fixed_bottom: usize,
}

impl From<Vec<String>> for Stage {
    fn from(vec: Vec<String>) -> Self {
        Self {
            vec,
            fixed_bottom: 0,
        }
    }
}

impl Stage {
    pub fn new(height: usize, string: String) -> Self {
        let stage = Self {
            vec: string.lines().map(String::from).collect(),
            fixed_bottom: 0,
        };
        stage.clear();
        stage
    }

    pub fn fix_bottom(&mut self, lines: usize) {
        self.fixed_bottom = lines;
    }

    pub fn set_vec(&mut self, vec: Vec<String>) {
        self.vec = vec;
        self.clear();
    }

    pub fn set_string(&mut self, new_string: String) {
        self.vec = new_string.lines().map(String::from).collect();
        self.clear();
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn clear(&self) {
        clear(self.len());
        move_up(self.len());
    }

    pub fn show(&self) {
        for line in &self.vec {
            fprintln!("{}", line)
        }
        move_up(self.len());
    }

    fn print_lines_from_bottom<D>(&self, bottom: usize, num_lines: usize, from: impl Iterator<Item=D>)
    where D: Display {
        for _ in 0..(bottom - num_lines) { move_down(1) }
        from.take(num_lines).for_each(|l| fprintln!("{}", l));
        move_up(bottom);
    }

    pub fn animate_up(&self) {
        if self.fixed_bottom > 0 {
            return self.animate_up_fixed_bottom()
        }

        for num_lines in 0..=self.len() {
            self.clear();
            self.print_lines_from_bottom(self.len(), num_lines, self.vec.iter());
            sleep(Duration::from_millis(50));
        }
    }

    fn animate_up_fixed_bottom(&self) {
        let lines = self.fixed_bottom;

        for num_lines in 0..=lines {
            self.clear();
            self.print_lines_from_bottom(self.len(), num_lines, self.vec.iter().skip(self.len() - lines));
            sleep(Duration::from_millis(50));
        }

        for num_lines in 0..=self.len()-lines {
            self.clear();
            self.print_lines_from_bottom(self.len() - lines, num_lines, self.vec.iter());
            move_down(self.len() - lines);
            for line in (self.len() - lines)..self.len() {
                fprintln!("{}", self.vec[line])
            }
            move_up(self.len());
            sleep(Duration::from_millis(50));
        }
    }

    pub fn animate_down(&self) {
        if self.fixed_bottom > 0 {
            return self.animate_down_fixed_bottom();
        }

        for num_lines in (0..=self.len()).rev() {
            self.clear();
            self.print_lines_from_bottom(self.len(), num_lines, self.vec.iter());
            sleep(Duration::from_millis(50));
        }
    }

    fn animate_down_fixed_bottom(&self) {
        let lines = self.fixed_bottom;

        for num_lines in (0..=self.len()-lines).rev() {
            self.clear();
            self.print_lines_from_bottom(self.len() - lines, num_lines, self.vec.iter());
            move_down(self.len() - lines);
            for line in (self.len() - lines)..self.len() {
                fprintln!("{}", self.vec[line])
            }
            move_up(self.len());
            sleep(Duration::from_millis(50));
        }

        for num_lines in (0..=lines).rev() {
            self.clear();
            self.print_lines_from_bottom(self.len(), num_lines, self.vec.iter().skip(self.len() - lines));
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
        let mut _byte = [0_u8];

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
        &mut Self, 0, fix_bottom(lines: usize);
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