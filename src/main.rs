// method call
// e.g. .map(m!(to_string)) = .map(|x| x.to_string())
macro_rules! m {
    ($method: ident) => {
        |x| x.$method()
    };
}

#[macro_use]
mod terminal;
mod game;
use game::Game;
mod words;

fn main() {
    let mut game = Game::new(words::COUNTRIES);
    loop { game.play_round(); }
}
