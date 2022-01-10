mod board;
mod emoji;
mod errors;

use board::{Board, Location, Player, Rotation};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let players = vec![Player::Player1, Player::Player2, Player::Player4];
    let mut board = Board::new(&mut rng, &players);

    println!("Board:\n{:?}", board);
    println!("Spare tile:\n{:?}", board.spare);
    board
        .insert_spare(Location(0, 1), Rotation::Clockwise180)
        .unwrap();

    println!("Board:\n{:?}", board);
    println!("Spare tile:\n{:?}", board.spare);
}
