use crate::board::{Item, Location, PlacedTile, Player, Tile};
use crate::errors::{GenericResult, WrongPlayer};
use crate::model::{Model, PlayerModel};
use std::collections::HashMap;
use std::convert::From;
use std::sync::mpsc::{Receiver, Sender};

pub struct PlayerSnapshot {
  player: Player,
  looking_for: Item,
}

impl From<&PlayerModel> for PlayerSnapshot {
  fn from(player: &PlayerModel) -> PlayerSnapshot {
    PlayerSnapshot {
      player: player.player,
      looking_for: player.current_card,
    }
  }
}

pub struct Snapshot {
  board: HashMap<Location, PlacedTile>,
  spare_tile: Tile,
  next_player: Player,
  players: Vec<PlayerSnapshot>,
}

impl From<&Model> for Snapshot {
  fn from(model: &Model) -> Snapshot {
    Snapshot {
      board: model.board.placed.clone(),
      spare_tile: model.board.spare,
      next_player: model.current_player,
      players: model
        .players
        .iter()
        .map(|player| PlayerSnapshot::from(player))
        .collect(),
    }
  }
}

#[derive(Debug)]
pub enum Command {
  NoOp,
  MovePlayer(Player, Location),
}

#[derive(Debug)]
pub struct CommandRequest {
  pub sent_by: Player,
  pub command: Command,
  pub respond: Sender<GenericResult<Snapshot>>,
}

pub fn run_controller(mut model: Model, command_rx: Receiver<CommandRequest>) {
  for request in command_rx {
    println!("Received command: {:?}", request);
    if request.sent_by != model.current_player {
      request
        .respond
        .send(Err(Box::new(WrongPlayer::new("It is not your turn"))))
        .unwrap();

      continue;
    }

    match request.command {
      Command::NoOp => request.respond.send(Ok(Snapshot::from(&model))).unwrap(),
      Command::MovePlayer(player, _) if player != model.current_player => {
        request
          .respond
          .send(Err(Box::new(WrongPlayer::new(
            "You cannot move another player",
          ))))
          .unwrap();
      }
      Command::MovePlayer(player, location) => match model.board.move_player(&player, &location) {
        Ok(_) => request.respond.send(Ok(Snapshot::from(&model))).unwrap(),
        Err(error) => request.respond.send(Err(error)).unwrap(),
      },
    }

    println!("Board: {:?}", model.board)
  }
}
