extern crate strum;
use crate::board::{Board, Item, Player};
use crate::errors::{GenericResult, WrongPlayer};
use rand::{seq::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Eq, PartialEq)]
pub enum TurnPhase {
  Move,
  InsertTile,
}

pub struct Model {
  pub board: Board,
  pub players: HashMap<Player, Cards>,
  pub current_player: Player,
  pub turn_phase: TurnPhase,
}

impl Model {
  /// Create a new model with a random board and random cards dealt to each player
  pub fn new<R: Rng>(
    rng: &mut R,
    players: &HashSet<Player>,
    starting_player: Player,
  ) -> GenericResult<Self> {
    if !players.contains(&starting_player) {
      return Err(Box::new(WrongPlayer::new("Starting player is not playing")));
    }

    let board = Board::new(rng, players);
    let mut player_cards: HashMap<Player, Cards> = players
      .iter()
      .map(|player| (*player, Cards::new()))
      .collect();

    // Divide the deck of items equally between the players
    // FIXME: Currently players will end up wih a different number of cards if the number of
    // Item enum values is not divisible by the number of players
    let mut deck: Vec<Item> = Item::iter().collect();
    deck.shuffle(rng);

    let mut current_player = starting_player;
    for card in deck {
      player_cards
        .get_mut(&current_player)
        .unwrap()
        .hidden_cards
        .push(card);

      current_player = next_player(&players, current_player).unwrap();
    }

    for (_, cards) in &mut player_cards {
      cards.draw_next();
    }

    Ok(Model {
      board,
      players: player_cards,
      current_player: starting_player,
      turn_phase: TurnPhase::InsertTile,
    })
  }

  pub fn current_player_cards(&self) -> &Cards {
    self.players.get(&self.current_player).unwrap()
  }

  pub fn current_player_cards_mut(&mut self) -> &mut Cards {
    self.players.get_mut(&self.current_player).unwrap()
  }

  pub fn end_turn(&mut self) {
    self.current_player =
      next_player(&self.players.keys().cloned().collect(), self.current_player).unwrap();
  }
}

pub struct Cards {
  pub current_card: Option<Item>,
  pub hidden_cards: Vec<Item>,
  pub found_cards: HashSet<Item>,
}

impl Cards {
  fn new() -> Self {
    Cards {
      current_card: None,
      hidden_cards: Vec::new(),
      found_cards: HashSet::new(),
    }
  }

  /// The current_card is found, move it to found_cards and take another from hidden_cards
  pub fn draw_next(&mut self) {
    if let Some(current_card) = self.current_card {
      self.found_cards.insert(current_card);
    }

    self.current_card = self.hidden_cards.pop();
  }
}

pub fn next_player(players: &HashSet<Player>, current_player: Player) -> GenericResult<Player> {
  let mut player = current_player;

  loop {
    player = match player {
      Player::Player1 => Player::Player2,
      Player::Player2 => Player::Player3,
      Player::Player3 => Player::Player4,
      Player::Player4 => Player::Player1,
    };

    if players.contains(&player) {
      return Ok(player);
    }

    if player == current_player {
      return Err(Box::new(WrongPlayer::new("Unable to find next player")));
    }
  }
}
