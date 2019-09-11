#[macro_use]
mod terminal;
mod game;
use game::Game;
mod words;
use terminal::animations::Stage;
use crate::terminal::Window;
use std::thread::sleep;
use std::time::Duration;
use crate::terminal::animations::StageWithEnterLine;

// method call
// e.g. .map(m!(to_string)) = .map(|x| x.to_string())
macro_rules! m {
    ($method: ident) => {
        |x| x.$method()
    };
}

fn main() {
    let vec = [
        "     *****     ",
        "   **     **   ",
        "  *         *  ",
        " *   hello   * ",
        " *           * ",
        " *           * ",
    ].iter().map(m!(to_string)).collect::<Vec<String>>();

    let s = StageWithEnterLine::from(vec);
    loop {
        s.animate_up();
        sleep(Duration::from_millis(200));
        s.prompt("PRESS ENTER TO CONTINUE");
        s.animate_down();
        sleep(Duration::from_millis(200));
    }



//    let w = Window::new(vec.len());
//    loop {
//        w.animate_in_up(&vec);
//        sleep(Duration::from_millis(200));
//        w.animate_out_down(&vec);
//        sleep(Duration::from_millis(200));
//    }

//    let s = Stage::new(5,
//                       "line 1\nline2\nline3\nline4\nENDline5".to_string());
//
//    std::thread::sleep(std::time::Duration::from_secs(1));
//    s.animate_up();
//    std::thread::sleep(std::time::Duration::from_secs(1));
//    s.clear();



//    let mut game = Game::new(words::COUNTRIES);
//    loop { game.play_round(); }
}
