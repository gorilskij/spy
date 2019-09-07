mod game;
use game::Game;
mod words;
mod terminal;

fn main() {
    let mut game = Game::new(words::COUNTRIES);
    loop { game.play_round(); }
}
