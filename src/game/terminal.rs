use std::io::{stdin, stdout};
use std::io::Write;

const E_ESCAPE: char = '\x1b'; // '\e' (ESC sequence)

pub fn flush() {
    stdout().flush().expect("an error occured on stdout.flush()");
}

pub fn clear_line() {
    print!("{}[2K\r", E_ESCAPE);
}

fn move_y(n: isize) {
    print!("{}[{}{}", E_ESCAPE, n.abs(), if n < 0 { 'A' } else { 'B' });
}

pub fn pre_clear(lines: usize) {
    assert!(lines < 1000);
    for _ in 0..lines {
        println!();
    }
    move_y(-(lines as isize));
}

fn enter_message(text: String) {
    print!("{}", text);
    flush();
    let mut _in = String::new();
    stdin().read_line(&mut _in).expect("an error occured on stdin.read_line()");
}

pub fn show_secret_text(pre: String, text: String, post: String) {
    let lines = text.split('\n').collect::<Vec<_>>();
    enter_message(pre);
    move_y(-1);
    clear_line();
    
    for line in &lines {
        println!("{}", line);
    }
    
    let lines_up = lines.len() as isize;
    
    enter_message(post);
    // TODO: invert loop, clear upwards
    move_y(-lines_up - 1);
    
    for _ in lines {
        clear_line();
        move_y(1);
    }
    
    clear_line();
    move_y(-lines_up);
}

pub fn ring() {
    print!("\x07");
}