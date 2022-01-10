mod board;
mod errors;

use board::{Board, Location, Player};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let players = vec![Player::Player1, Player::Player2, Player::Player4];
    let mut board = Board::new(&mut rng, &players);

    println!("Board:\n{:?}", board);
    println!("Spare tile:\n{:?}", board.spare);

    board
        .move_player(&Player::Player1, &Location(2, 2))
        .unwrap();

    println!("Board:\n{:?}", board);
    println!("Spare tile:\n{:?}", board.spare);
}
