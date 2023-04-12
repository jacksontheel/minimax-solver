use std::time::Instant;

use crate::{
    solver::{minimax::make_move, gamestate::GameState},
    tictactoe::TicTacToeGameState,
};

mod solver;
mod tictactoe;

fn main() {
    let now = Instant::now();

    let board = TicTacToeGameState::new();
    let result = make_move(&board, 10, true);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", result.to_string());
}
