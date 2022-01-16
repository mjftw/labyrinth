use crate::board::{Board, Item, Player};

pub struct Model {
  pub board: Board,
  pub players: Vec<PlayerModel>,
  pub current_player: Player,
}

pub struct PlayerModel {
  pub player: Player,
  pub current_card: Item,
  pub hidden_cards: Vec<Item>,
  pub found_cards: Vec<Item>,
}
