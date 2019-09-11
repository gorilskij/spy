#[macro_export]
macro_rules! fprint {
    ($( $arg: expr ),* $(,)?) => {{
        print!($( $arg ),*);
        use std::io::Write;
        let _ = std::io::stdout().flush().expect("failed to flush");
    }};
}

#[macro_export]
macro_rules! fprintln {
    ($( $arg: expr ),* $(,)?) => {{
        println!($( $arg ),*);
        use std::io::Write;
        let _ = std::io::stdout().flush().expect("failed to flush");
    }};
}

use std::io::{stdin, stdout};
use std::io::Write;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;

pub mod animations;

const E_ESCAPE: char = '\x1b'; // '\e' (ESC sequence)

pub struct Window {
    height: usize,
}

impl Window {
    pub fn new(height: usize) -> Self {
        let it = Self { height };
        it.clear();
        it
    }

    fn back_up(&self) {
        move_up(self.height - 1);
    }

    fn each_line(&self, f: impl Fn(usize)) {
        for i in 0..(self.height - 1) {
            f(i);
            fprintln!();
        }
        f(self.height - 1);
        self.back_up();
    }

    pub fn clear(&self) {
        self.each_line(|_| clear_line());
    }

    pub fn show_domain(&self) {
        self.each_line(|_| fprint!("\r> "));
    }

    fn show_lines_from_below(&self, text: &Vec<String>, num_lines: usize) {
        self.each_line(|mut i| {
            clear_line();
            i += 1;
            if i >= self.height - num_lines {
                fprint!("{}", text[i - (self.height - num_lines)])
            }
        });
        sleep(Duration::from_millis(100));
    }

    pub fn animate_in_up(&self, text: &Vec<String>) {
        for num_lines in 0..self.height {
            self.show_lines_from_below(text, num_lines);
            sleep(Duration::from_millis(100));
        }
    }

    pub fn animate_out_down(&self, text: &Vec<String>) {
        for num_lines in (0..self.height).rev() {
            self.show_lines_from_below(text, num_lines);
            sleep(Duration::from_millis(100));
        }
    }
}

pub fn save_cursor() {
    fprint!("{}[s", E_ESCAPE);
}

pub fn restore_cursor() {
    fprint!("{}[u", E_ESCAPE);
}

pub fn clear_line() {
    fprint!("{}[2K\r", E_ESCAPE);
}

fn move_up(n: usize) {
    fprint!("{}[{}A", E_ESCAPE, n);
}

fn move_down(n: usize) {
    fprint!("{}[{}B", E_ESCAPE, n);
}

pub fn clear(lines: usize) {
    save_cursor();
    for _ in 0..lines {
        clear_line();
        fprintln!();
    }
    restore_cursor();
}

fn enter_message(text: String) {
    fprint!("{}", text);
    let mut _in = String::new();
    stdin().read_line(&mut _in).expect("an error occured on stdin.read_line()");
}

pub fn show_secret_text(pre: String, text: String, post: String) {
    let lines = text.split('\n').collect::<Vec<_>>();
    enter_message(pre);
    move_up(1);
    clear_line();
    
    for line in &lines {
        fprintln!("{}", line);
    }
    
    let lines_up = lines.len();
    
    enter_message(post);
    // TODO: invert loop, clear upwards
    move_up(lines_up + 1);
    
    for _ in lines {
        clear_line();
        move_down(1);
    }
    
    clear_line();
    move_up(lines_up);
}

pub fn ring() {
    fprint!("\x07");
}

pub fn temporary_text(text: impl Into<String>, millis: u64) {
    fprint!("{}", text.into());
    std::thread::sleep(std::time::Duration::from_millis(millis));
    clear_line();
}