pub mod board;
pub mod color;
pub mod game;
pub mod movedata;
pub mod movegen;
pub mod moverules;
pub mod occupancy;
pub mod piece;
pub mod square;
pub mod standardstart;
pub mod title;

/*
//Loop for testing with terminal interface (convert lib.rs to main.rs to use)
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut game = game::Game::new();
    loop {
        game::print_board_state(&game.board);
        //stdin.lock().read_line(&mut input);
        let input = stdin.lock().lines().next().unwrap().unwrap();
        game.make_move_from_notation(input);
    }
}*/
