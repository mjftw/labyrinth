use crate::board::{Item, Location, PlacedTile, Player, Rotation, Tile};
use crate::errors::{GenericError, GenericResult, TurnError, WrongPlayer};
use crate::model::{Cards, Model, TurnPhase};
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::sync::mpsc::{Receiver, Sender};

pub struct CardsSnapshot {
  found: HashSet<Item>,
  num_hidden_cards: u32,
}

impl From<&Cards> for CardsSnapshot {
  fn from(cards: &Cards) -> CardsSnapshot {
    CardsSnapshot {
      found: cards.found_cards.clone(),
      num_hidden_cards: cards.hidden_cards.len() as u32,
    }
  }
}

pub struct Snapshot {
  board: HashMap<Location, PlacedTile>,
  spare_tile: Tile,
  next_player: Player,
  looking_for: Option<Item>,
  players: HashMap<Player, CardsSnapshot>,
}

impl Snapshot {
  /// Create a new snapshot of the game state, showing only info visible to `player`
  fn for_player(model: &Model, player: Player) -> Snapshot {
    Snapshot {
      board: model.board.placed.clone(),
      spare_tile: model.board.spare,
      next_player: model.current_player,
      looking_for: model.current_player_cards().current_card,
      players: model
        .players
        .iter()
        .map(|(player, cards)| (*player, CardsSnapshot::from(cards)))
        .collect(),
    }
  }
}

#[derive(Debug)]
pub enum Command {
  NoOp,
  MovePlayer(Player, Location),
  InsertTile(Location, Rotation),
}

type SnapshotSender = Sender<GenericResult<Snapshot>>;

#[derive(Debug)]
pub struct CommandRequest {
  pub sent_by: Player,
  pub command: Command,
  pub respond: SnapshotSender,
}

pub fn run_controller(mut model: Model, command_rx: Receiver<CommandRequest>) {
  for request in command_rx {
    println!("{:?} sent command {:?}", request.sent_by, request.command);

    if request.sent_by != model.current_player {
      respond_error(&request, Box::new(WrongPlayer::new("It is not your turn")));
      continue;
    }

    match request.command {
      Command::NoOp => respond_snapshot(&request, &model),
      Command::MovePlayer(_, _) if model.turn_phase != TurnPhase::Move => respond_error(
        &request,
        Box::new(TurnError::new(
          "It is not time to move, you must first insert the tile",
        )),
      ),
      Command::MovePlayer(player, _) if player != model.current_player => respond_error(
        &request,
        Box::new(WrongPlayer::new("You cannot move another player")),
      ),
      Command::MovePlayer(player, location) => {
        do_then_respond(&mut model, &request, &mut |model| {
          move_player(player, location, model)
        })
      }
      Command::InsertTile(_, _) if model.turn_phase != TurnPhase::InsertTile => respond_error(
        &request,
        Box::new(TurnError::new(
          "It is not time to insert the tile, you must move",
        )),
      ),
      Command::InsertTile(location, rotation) => {
        do_then_respond(&mut model, &request, &mut |model| {
          Ok(model.board.insert_spare(location, rotation)?)
        })
      }
    }
  }
}

fn respond_snapshot(request: &CommandRequest, model: &Model) {
  request
    .respond
    .send(Ok(Snapshot::for_player(&model, request.sent_by)))
    .unwrap()
}

fn respond_error(request: &CommandRequest, error: GenericError) {
  request.respond.send(Err(error)).unwrap();
}

/// Call the function and respond to the request.
/// If the function returns an `Ok(_)` then send a snapshot of the model from the players view.
/// If the function returns an `Err(error)` then forward this `error` on to the command requester.
fn do_then_respond<F: FnMut(&mut Model) -> GenericResult<()>>(
  model: &mut Model,
  request: &CommandRequest,
  f: &mut F,
) {
  match f(model) {
    Ok(_) => respond_snapshot(request, model),
    Err(error) => respond_error(request, error),
  }
}

/// Move a player across the board and end their turn
fn move_player(player: Player, location: Location, model: &mut Model) -> GenericResult<()> {
  model.board.move_player(&player, &location)?;

  let player_cards = model.current_player_cards();

  if player_cards.current_card.is_some()
    && player_cards.current_card == model.board.item_at(&location).unwrap()
  {
    // Player has found the item they're looking for, draw the next item card
    model.current_player_cards_mut().draw_next();
  }

  model.end_turn();

  Ok(())
}
