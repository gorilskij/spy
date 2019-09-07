use std::io::{stdin, stdout};
use std::io::Write;
extern crate rand;
use rand::Rng;

fn e_escape() -> char { char::from(27) }

fn clear_line() {
    print!("{}[2K\r", e_escape());
}

fn move_y(n: isize) {
    print!("{}[{}{}", e_escape(), n.abs(), if n < 0 { 'A' } else { 'B' });
}

fn enter_message(text: String) {
    print!("{}", text);
    stdout().flush().expect("an error occured on stdout.flush()");
    let mut _in = String::new();
    stdin().read_line(&mut _in).expect("an error occured on stdin.read_line()");
}

fn show_secret_text(pre: String, text: String, post: String) {
    let lines = text.split('\n').collect::<Vec<_>>();
    enter_message(pre);
    move_y(-1);
    clear_line();
    
    for line in &lines {
        println!("{}", line);
    }
    
    enter_message(post);
    move_y(-(lines.len() as isize + 1));
    
    for _ in lines {
        clear_line();
        move_y(1);
    }
    
    clear_line();
    move_y(1);
}

fn rand_word() -> &'static str {
    "hello"
}

fn spy_round() {
    let spy = rand::thread_rng().gen_range(0, 2);
    let word = rand_word();
    
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
}

fn main() {
    spy_round();
}
