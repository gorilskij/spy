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

pub mod animations;

const E_ESCAPE: char = '\x1b'; // '\e' (ESC sequence)

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
    for _ in 0..lines {
        clear_line();
        fprintln!();
    }
}

pub fn ring() {
    fprint!("\x07");
}