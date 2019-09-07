use std::time::{Instant, Duration};
use std::thread::sleep;
use crate::terminal::{clear_line, flush, ring};

fn print_time(seconds: u64) {
    print!("{:0>2}:{:0>2}", seconds / 60, seconds % 60);
}

pub fn run_timer(seconds: u64) {
    println!("You have {}s", seconds);
    let duration = Duration::from_secs(seconds);
    let start = Instant::now();
    
    loop {
        let elapsed = start.elapsed();
        if duration < elapsed { break }
        clear_line();
        print_time((duration - elapsed).as_secs());
        flush();
        sleep(Duration::from_millis(333));
    }
    
    println!("TIME'S UP");
    ring();
    ring();
}