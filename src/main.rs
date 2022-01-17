mod board;
mod controller;
mod emoji;
mod errors;
mod model;

use controller::{run_controller, Command, CommandRequest};
use model::Model;
use std::collections::HashSet;

use std::sync::mpsc::channel;
use std::thread;

use board::{Board, Location, Player};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let players: HashSet<Player> = [Player::Player1, Player::Player2, Player::Player4]
        .into_iter()
        .collect();

    let current_player = Player::Player1;

    let model = Model::new(&mut rng, &players, current_player).unwrap();

    println!("Board:\n{:?}", model.board);
    println!("Spare tile:\n{:?}", model.board.spare);

    let (controller_tx, controller_rx) = channel();

    let controller_handle = thread::spawn(move || run_controller(model, controller_rx));

    let (respond_tx, respond_rx) = channel();

    let request = CommandRequest {
        sent_by: current_player,
        command: Command::MovePlayer(current_player, Location(2, 1)),
        respond: respond_tx,
    };

    controller_tx.send(request).unwrap();

    let response = respond_rx.recv().unwrap().unwrap();

    controller_handle.join().unwrap();
}
