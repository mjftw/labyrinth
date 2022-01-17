extern crate strum_macros;
extern crate unicode_width;
use crate::emoji::Emoji;
use crate::errors::{GenericResult, LocationError, MoveError};

use itertools::Itertools;
use rand::{
  distributions::{Distribution, Standard},
  seq::SliceRandom,
  Rng,
};
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::fmt;
use std::iter::Iterator;

#[derive(Hash, PartialEq, Eq, Copy, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum Item {
  Chest,
  Gnome,
  Dragon,
  Unicorn,
  Ghost,
  Candle,
  Cat,
  Keys,
  Book,
  Spider,
  Crown,
  Sword,
  Goblet,
  Mouse,
  Ring,
  Potion,
  Beetle,
  Owl,
  Gem,
  Genie,
  Bat,
  Sack,
  Helmet,
  Lizard,
}

impl Emoji for Item {
  fn emoji(&self) -> &str {
    match self {
      Item::Chest => "📦",
      Item::Gnome => "👦",
      Item::Dragon => "🐉",
      Item::Unicorn => "🦄",
      Item::Ghost => "👻",
      Item::Candle => "🕯️",
      Item::Cat => "🐱",
      Item::Keys => "🗝",
      Item::Book => "📕",
      Item::Spider => "🕷",
      Item::Crown => "👑",
      Item::Sword => "🗡️",
      Item::Goblet => "🏆",
      Item::Mouse => "🐭",
      Item::Ring => "💍",
      Item::Potion => "⚗️",
      Item::Beetle => "🐞",
      Item::Owl => "🦉",
      Item::Gem => "💎",
      Item::Genie => "🧞‍♀️",
      Item::Bat => "🦇",
      Item::Sack => "💰",
      Item::Helmet => "🧢",
      Item::Lizard => "🦎",
    }
  }
}

impl fmt::Debug for Item {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.emoji())
  }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum Player {
  Player1,
  Player2,
  Player3,
  Player4,
}

impl Emoji for Player {
  fn emoji(&self) -> &str {
    match self {
      Player::Player1 => "♠",
      Player::Player2 => "♥",
      Player::Player3 => "♦",
      Player::Player4 => "♣",
    }
  }
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.emoji())
  }
}

#[derive(Copy, Clone)]
pub enum TileMarking {
  Item(Item),
  PlayerStart(Player),
}

impl Emoji for TileMarking {
  fn emoji(&self) -> &str {
    match self {
      TileMarking::Item(item) => item.emoji(),
      TileMarking::PlayerStart(Player::Player1) => "♤", //red
      TileMarking::PlayerStart(Player::Player2) => "♡", //blue
      TileMarking::PlayerStart(Player::Player3) => "♢", //yellow
      TileMarking::PlayerStart(Player::Player4) => "♧", //green
    }
  }
}

impl fmt::Display for TileMarking {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.emoji())
  }
}

#[derive(Copy, Clone)]
pub struct Tile {
  pub marking: Option<TileMarking>,
  pub path_up: bool,
  pub path_right: bool,
  pub path_down: bool,
  pub path_left: bool,
}

impl From<&PlacedTile> for Tile {
  fn from(placed_tile: &PlacedTile) -> Self {
    let (path_up, path_right, path_down, path_left) = match placed_tile.rotation {
      Rotation::Zero => (
        placed_tile.tile.path_up,
        placed_tile.tile.path_right,
        placed_tile.tile.path_down,
        placed_tile.tile.path_left,
      ),
      Rotation::Clockwise90 => (
        placed_tile.tile.path_left,
        placed_tile.tile.path_up,
        placed_tile.tile.path_right,
        placed_tile.tile.path_down,
      ),
      Rotation::Clockwise180 => (
        placed_tile.tile.path_down,
        placed_tile.tile.path_left,
        placed_tile.tile.path_up,
        placed_tile.tile.path_right,
      ),
      Rotation::Clockwise270 => (
        placed_tile.tile.path_right,
        placed_tile.tile.path_down,
        placed_tile.tile.path_left,
        placed_tile.tile.path_up,
      ),
    };

    Tile {
      marking: placed_tile.tile.marking,
      path_up,
      path_right,
      path_down,
      path_left,
    }
  }
}

impl fmt::Debug for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let placed_tile: PlacedTile = PlacedTile::from(self);
    PlacedTile::fmt(&placed_tile, f)
  }
}

impl Tile {
  const CORNER_RIGHT_DOWN: Tile = Tile {
    marking: None,
    path_up: false,
    path_right: true,
    path_down: true,
    path_left: false,
  };

  const CORNER_LEFT_DOWN: Tile = Tile {
    marking: None,
    path_up: false,
    path_right: false,
    path_down: true,
    path_left: true,
  };

  const CORNER_LEFT_UP: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: false,
    path_down: false,
    path_left: true,
  };

  const CORNER_RIGHT_UP: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: true,
    path_down: false,
    path_left: false,
  };

  const TEE_LEFT: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: false,
    path_down: true,
    path_left: true,
  };

  const TEE_RIGHT: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: true,
    path_down: true,
    path_left: false,
  };

  const TEE_UP: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: true,
    path_down: false,
    path_left: true,
  };

  const TEE_DOWN: Tile = Tile {
    marking: None,
    path_up: false,
    path_right: true,
    path_down: true,
    path_left: true,
  };

  const LINE_VERTICAL: Tile = Tile {
    marking: None,
    path_up: true,
    path_right: false,
    path_down: true,
    path_left: false,
  };

  const LINE_HORIZONTAL: Tile = Tile {
    marking: None,
    path_up: false,
    path_right: true,
    path_down: false,
    path_left: true,
  };
}

#[derive(Copy, Clone)]
pub enum Rotation {
  Zero,
  Clockwise90,
  Clockwise180,
  Clockwise270,
}

impl Distribution<Rotation> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
    match rng.gen_range(0..=3) {
      0 => Rotation::Zero,
      1 => Rotation::Clockwise90,
      2 => Rotation::Clockwise180,
      _ => Rotation::Clockwise270,
    }
  }
}

#[derive(Clone)]
pub struct PlacedTile {
  pub tile: Tile,
  pub rotation: Rotation,
  pub players: HashSet<Player>,
}

impl From<&Tile> for PlacedTile {
  fn from(tile: &Tile) -> PlacedTile {
    PlacedTile {
      tile: *tile,
      rotation: Rotation::Zero,
      players: HashSet::new(),
    }
  }
}

impl fmt::Debug for PlacedTile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let tile = Tile::from(self);

    let w = "▒";
    let p = "░";

    let player1 = if self.players.contains(&Player::Player1) {
      format!("{}", Player::Player1)
    } else {
      p.to_string()
    };

    let player2 = if self.players.contains(&Player::Player2) {
      format!("{}", Player::Player2)
    } else {
      p.to_string()
    };

    let player3 = if self.players.contains(&Player::Player3) {
      format!("{}", Player::Player3)
    } else {
      p.to_string()
    };

    let player4 = if self.players.contains(&Player::Player4) {
      format!("{}", Player::Player4)
    } else {
      p.to_string()
    };

    write!(
      f,
      "{}{}{}\n\
      {}{}{}\n\
      {}{}{}{}{}{}\n\
      {}{}{}",
      w,
      if tile.path_up {
        p.repeat(4)
      } else {
        w.repeat(4)
      },
      w,
      if tile.path_left { p } else { w },
      if tile.marking.is_some() {
        let marking = &tile.marking.unwrap();

        format!("{}{}", marking.emoji(), p.repeat(4 - marking.emoji_width()))
      } else {
        p.repeat(4)
      },
      if tile.path_right { p } else { w },
      if tile.path_left { p } else { w },
      player1,
      player2,
      player3,
      player4,
      if tile.path_right { p } else { w },
      w,
      if tile.path_down {
        p.repeat(4)
      } else {
        w.repeat(4)
      },
      w,
    )
  }
}

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Location(pub usize, pub usize);

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

/// A board containing all tiles placed on the board and the spare extra tile
pub struct Board {
  pub placed: HashMap<Location, PlacedTile>,
  pub spare: Tile,
  graph: BoardGraph,
}

pub struct BoardIter<'a> {
  board: &'a Board,
  locations: Box<dyn Iterator<Item = Location>>,
}

impl<'a> Iterator for BoardIter<'a> {
  type Item = &'a PlacedTile;
  fn next(&mut self) -> Option<Self::Item> {
    self
      .locations
      .next()
      .map(|location| self.board.placed.get(&location).unwrap())
  }
}

impl<'a> IntoIterator for &'a Board {
  type Item = &'a PlacedTile;
  type IntoIter = BoardIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    BoardIter {
      board: self,
      locations: Board::locations(),
    }
  }
}

impl fmt::Debug for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let col_sep = " ";
    let row_sep = " ";
    let tile_line_strs: Vec<Vec<String>> = self
      .into_iter()
      .map(|tile| format!("{:?}", tile))
      .map(|tile_strings| {
        tile_strings
          .lines()
          .map(String::from)
          .collect::<Vec<String>>()
      })
      .collect();

    let board_row_strs: Vec<String> = tile_line_strs
      .chunks(7)
      .map(|row_strs| {
        let num_cols = row_strs.first().unwrap().len();
        let col_strs: Vec<String> = (0..num_cols)
          .map(|col| {
            row_strs
              .iter()
              .map(|tile_str| tile_str.iter().nth(col).unwrap())
              .join(col_sep)
          })
          .collect();

        col_strs.join("\n")
      })
      .collect();

    let num_col_chars = *board_row_strs
      .first()
      .unwrap()
      .lines()
      .map(|line| line.chars().count())
      .take(1)
      .collect::<Vec<usize>>()
      .first()
      .unwrap();

    let board_str: String = board_row_strs
      .iter()
      .intersperse(&row_sep.repeat(num_col_chars))
      .join("\n");

    write!(f, "{}", board_str)
  }
}

impl Board {
  /// The tiles that are fixed to the board and cannot be moved or rotated
  const FIXED_TILES: [(Location, Tile); 16] = [
    (
      Location(0, 0),
      Tile {
        marking: Some(TileMarking::PlayerStart(Player::Player1)),
        ..Tile::CORNER_RIGHT_DOWN
      },
    ),
    (
      Location(2, 0),
      Tile {
        marking: Some(TileMarking::Item(Item::Goblet)),
        ..Tile::TEE_DOWN
      },
    ),
    (
      Location(4, 0),
      Tile {
        marking: Some(TileMarking::Item(Item::Sword)),
        ..Tile::TEE_DOWN
      },
    ),
    (
      Location(6, 0),
      Tile {
        marking: Some(TileMarking::PlayerStart(Player::Player2)),
        ..Tile::CORNER_LEFT_DOWN
      },
    ),
    (
      Location(0, 2),
      Tile {
        marking: Some(TileMarking::Item(Item::Sack)),
        ..Tile::TEE_RIGHT
      },
    ),
    (
      Location(2, 2),
      Tile {
        marking: Some(TileMarking::Item(Item::Keys)),
        ..Tile::TEE_RIGHT
      },
    ),
    (
      Location(4, 2),
      Tile {
        marking: Some(TileMarking::Item(Item::Gem)),
        ..Tile::TEE_DOWN
      },
    ),
    (
      Location(6, 2),
      Tile {
        marking: Some(TileMarking::Item(Item::Helmet)),
        ..Tile::TEE_LEFT
      },
    ),
    (
      Location(0, 4),
      Tile {
        marking: Some(TileMarking::Item(Item::Book)),
        ..Tile::TEE_RIGHT
      },
    ),
    (
      Location(2, 4),
      Tile {
        marking: Some(TileMarking::Item(Item::Crown)),
        ..Tile::TEE_UP
      },
    ),
    (
      Location(4, 4),
      Tile {
        marking: Some(TileMarking::Item(Item::Chest)),
        ..Tile::TEE_LEFT
      },
    ),
    (
      Location(6, 4),
      Tile {
        marking: Some(TileMarking::Item(Item::Candle)),
        ..Tile::TEE_LEFT
      },
    ),
    (
      Location(0, 6),
      Tile {
        marking: Some(TileMarking::PlayerStart(Player::Player3)),
        ..Tile::CORNER_RIGHT_UP
      },
    ),
    (
      Location(2, 6),
      Tile {
        marking: Some(TileMarking::Item(Item::Potion)),
        ..Tile::TEE_UP
      },
    ),
    (
      Location(4, 6),
      Tile {
        marking: Some(TileMarking::Item(Item::Ring)),
        ..Tile::TEE_UP
      },
    ),
    (
      Location(6, 6),
      Tile {
        marking: Some(TileMarking::PlayerStart(Player::Player4)),
        ..Tile::CORNER_LEFT_UP
      },
    ),
  ];

  /// The tiles that are free to be placed or rotated
  const FREE_TILES: [Tile; 34] = [
    Tile {
      marking: Some(TileMarking::Item(Item::Spider)),
      ..Tile::CORNER_RIGHT_DOWN
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Ghost)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Unicorn)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Gnome)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Cat)),
      ..Tile::CORNER_RIGHT_DOWN
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Owl)),
      ..Tile::CORNER_RIGHT_DOWN
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Genie)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Mouse)),
      ..Tile::CORNER_LEFT_DOWN
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Lizard)),
      ..Tile::CORNER_RIGHT_DOWN
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Bat)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Dragon)),
      ..Tile::TEE_UP
    },
    Tile {
      marking: Some(TileMarking::Item(Item::Beetle)),
      ..Tile::CORNER_RIGHT_DOWN
    },
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::LINE_VERTICAL,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
    Tile::CORNER_RIGHT_UP,
  ];

  pub fn locations() -> Box<dyn Iterator<Item = Location>> {
    let mut l: Vec<_> = (0..7)
      .permutations(2)
      .chain((0..7).map(|i| vec![i, i]))
      .collect();
    l.sort_by_key(|a| match &a[..] {
      &[x, y] => (y, x),
      _ => (0, 0),
    });

    Box::new(l.into_iter().filter_map(|a| match &a[..] {
      &[x, y] => Some(Location(x, y)),
      _ => None,
    }))
  }

  pub fn neighbors(&self, at: &Location) -> Result<Vec<Location>, LocationError> {
    // Check location exists on board
    let here = Tile::from(self.placed.get(at).ok_or(at)?);

    // No neighbors off the edge of the board
    let up = if at.1 > 0 {
      let up_at = Location(at.0, at.1 - 1);
      self
        .placed
        .get(&up_at)
        .map(|tile| {
          if here.path_up && Tile::from(tile).path_down {
            Some(up_at)
          } else {
            None
          }
        })
        .flatten()
    } else {
      None
    };

    // No neighbors off the edge of the board
    let left = if at.0 > 0 {
      let left_at = Location(at.0 - 1, at.1);
      self
        .placed
        .get(&left_at)
        .map(|tile| {
          if here.path_left && Tile::from(tile).path_right {
            Some(left_at)
          } else {
            None
          }
        })
        .flatten()
    } else {
      None
    };

    // No neighbors off the edge of the board
    let down = if at.1 + 1 < 6 {
      let down_at = Location(at.0, at.1 + 1);
      self
        .placed
        .get(&down_at)
        .map(|tile| {
          if here.path_down && Tile::from(tile).path_up {
            Some(down_at)
          } else {
            None
          }
        })
        .flatten()
    } else {
      None
    };

    // No neighbors off the edge of the board
    let right = if at.0 + 1 < 6 {
      let right_at = Location(at.0 + 1, at.1);
      self
        .placed
        .get(&right_at)
        .map(|tile| {
          if here.path_right && Tile::from(tile).path_left {
            Some(right_at)
          } else {
            None
          }
        })
        .flatten()
    } else {
      None
    };

    let neighbors: Vec<Location> = [up, down, left, right].iter().filter_map(|l| *l).collect();

    Ok(neighbors)
  }

  pub fn players(&self) -> HashMap<Location, Player> {
    self
      .placed
      .iter()
      .flat_map(|(location, tile)| tile.players.iter().map(|player| (*location, *player)))
      .collect()
  }

  pub fn move_player(&mut self, player: &Player, move_to: &Location) -> GenericResult<()> {
    let players = self.players();

    let current_location = *players
      .iter()
      .filter_map(|(l, p)| if p == player { Some(*l) } else { None })
      .collect::<Vec<_>>()
      .first()
      .ok_or(MoveError::new("Player not found on board"))?;

    if self.graph.is_connected(&current_location, move_to)? {
      // Take player off previous tile
      self
        .placed
        .get_mut(&current_location)
        .ok_or(LocationError::from(move_to))?
        .players
        .remove(player);

      // Add player to new tile
      self
        .placed
        .get_mut(move_to)
        .ok_or(LocationError::from(move_to))?
        .players
        .insert(*player);
      Ok(())
    } else {
      Err(Box::new(MoveError::new("No path to destination")))
    }
  }

  /// Check whether placing a tile with a certain rotation at a certain location is allowed
  /// No tile can be placed so that one of the openings leads off the board
  fn tile_placement_ok(location: &Location, placed_tile: &PlacedTile) -> bool {
    let rotated_tile = Tile::from(placed_tile);
    match location {
      // top left corner
      Location(0, 0) => !rotated_tile.path_up && !rotated_tile.path_left,
      // top right corner
      Location(6, 0) => !rotated_tile.path_up && !rotated_tile.path_right,
      // bottom right corner
      Location(6, 6) => !rotated_tile.path_down && !rotated_tile.path_right,
      // bottom left corner
      Location(0, 6) => !rotated_tile.path_down && !rotated_tile.path_left,
      // bottom edge corner
      Location(_, 6) => !rotated_tile.path_down,
      // top edge corner
      Location(_, 0) => !rotated_tile.path_up,
      // left edge corner
      Location(0, _) => !rotated_tile.path_left,
      // right edge corner
      Location(6, _) => !rotated_tile.path_right,
      _ => true,
    }
  }

  // Get the on the tile at the location, if there is one
  pub fn item_at(&self, location: &Location) -> GenericResult<Option<Item>> {
    if let Some(tile) = self.placed.get(location) {
      match tile.tile.marking {
        Some(TileMarking::Item(item)) => Ok(Some(item)),
        _ => Ok(None),
      }
    } else {
      Err(Box::new(LocationError::from(location)))
    }
  }

  /// Create a new board including the fixed tiles, with free tiles placed using the random number generator
  pub fn new<R: Rng>(rng: &mut R, players: &HashSet<Player>) -> Board {
    let fixed_tiles = Board::FIXED_TILES.clone().map(|(location, tile)| {
      (
        location,
        PlacedTile {
          tile,
          rotation: Rotation::Zero,
          players: tile
            .marking
            .map(|marking| match marking {
              TileMarking::PlayerStart(Player::Player1) if players.contains(&Player::Player1) => {
                HashSet::from([Player::Player1])
              }
              TileMarking::PlayerStart(Player::Player2) if players.contains(&Player::Player2) => {
                HashSet::from([Player::Player2])
              }
              TileMarking::PlayerStart(Player::Player3) if players.contains(&Player::Player3) => {
                HashSet::from([Player::Player3])
              }
              TileMarking::PlayerStart(Player::Player4) if players.contains(&Player::Player4) => {
                HashSet::from([Player::Player4])
              }
              _ => HashSet::new(),
            })
            .unwrap_or(HashSet::new()),
        },
      )
    });

    let mut free_tiles: Vec<PlacedTile> = Board::FREE_TILES
      .clone()
      .into_iter()
      .map(|tile| PlacedTile {
        tile,
        rotation: rng.gen(),
        players: HashSet::new(),
      })
      .collect();

    let mut free_locations: Vec<Location> = Board::locations()
      .filter(|location| {
        !fixed_tiles
          .iter()
          .map(|(location, _)| location)
          .contains(location)
      })
      .collect();

    // There should always be 1 more free tile than free location
    assert_eq!(free_locations.len(), free_tiles.len() - 1);

    free_tiles.shuffle(rng);
    free_locations.shuffle(rng);

    let extra_tile = free_tiles.pop().unwrap().tile;
    let mut placed_tiles: Vec<_> = free_locations.into_iter().zip(free_tiles).collect();

    // Rotate any tiles that would have an invalid placement until placement okay
    for mut placed_tile in &mut placed_tiles {
      while !Board::tile_placement_ok(&placed_tile.0, &placed_tile.1) {
        (placed_tile.1).rotation = rng.gen();
      }
    }

    let mut board = Board {
      placed: fixed_tiles.into_iter().chain(placed_tiles).collect(),
      spare: extra_tile,
      graph: BoardGraph::empty(),
    };
    board.graph = BoardGraph::from(&board);

    board
  }

  /// Generic helper function for rotating a row or column
  fn rotate_common(
    &mut self,
    idx: usize,
    tile_rotation: Rotation,
    reverse: bool,
    idx_is_x: bool,
  ) -> Result<(), LocationError> {
    let push_in_at = match (reverse, idx_is_x) {
      (false, false) => Location(0, idx),
      (false, true) => Location(idx, 0),
      (true, false) => Location(6, idx),
      (true, true) => Location(idx, 6),
    };

    let push_out_at = match (reverse, idx_is_x) {
      (false, false) => Location(6, idx),
      (false, true) => Location(idx, 6),
      (true, false) => Location(0, idx),
      (true, true) => Location(idx, 0),
    };

    let mut to_push_in = PlacedTile {
      tile: self.spare,
      rotation: tile_rotation,
      players: HashSet::new(),
    };

    // Ensure rotating the spare tile in won't break the board
    if !Board::tile_placement_ok(&push_in_at, &to_push_in) {
      return Err(LocationError::new(
        "Tile cannot be inserted here with this rotation",
      ));
    }

    let pushed_out = self
      .placed
      .remove(&push_out_at)
      .ok_or(LocationError::from(&push_out_at))?;

    // If a player is pushed off the board then they are moved to the newly inserted tile
    to_push_in.players = pushed_out.players;

    let mut moving_tile = self.placed.remove(&push_in_at);

    let iter: Box<dyn Iterator<Item = usize>> = if reverse {
      Box::new((0..6).rev())
    } else {
      Box::new(1..7)
    };

    for i in iter {
      let move_to = if idx_is_x {
        Location(idx, i)
      } else {
        Location(i, idx)
      };
      moving_tile = self.placed.insert(move_to, moving_tile.unwrap());
    }

    self.placed.insert(push_in_at, to_push_in);
    self.spare = pushed_out.tile;

    Ok(())
  }

  /// Rotate row y left, replacing the rightmost tile with the spare tile
  fn rotate_left(&mut self, y: usize, tile_rotation: Rotation) -> Result<(), LocationError> {
    self.rotate_common(y, tile_rotation, true, false)
  }

  /// Rotate row y right, replacing the leftmost tile with the spare tile
  fn rotate_right(&mut self, y: usize, tile_rotation: Rotation) -> Result<(), LocationError> {
    self.rotate_common(y, tile_rotation, false, false)
  }

  /// Rotate column x up, replacing the bottommost tile with the spare tile
  fn rotate_up(&mut self, x: usize, tile_rotation: Rotation) -> Result<(), LocationError> {
    self.rotate_common(x, tile_rotation, true, true)
  }

  /// Rotate column x down, replacing the topmost tile with the spare tile
  fn rotate_down(&mut self, x: usize, tile_rotation: Rotation) -> Result<(), LocationError> {
    self.rotate_common(x, tile_rotation, false, true)
  }

  /// Try to insert the extra tile at a given location, sliding all the tiles in the row/column by 1.
  /// Inserting a tile pushes the tile opposite off the board, which becomes the new extra tile.
  /// Returns Ok(()) if insertion was possible, and Err(()) if not.
  /// Valid insertion locations are:
  /// (1,0), (3,0), (5,0), (6,1), (6,3), (6,5), (1,6), (3,6), (5,6), (0,1), (0,3), (0,5),
  pub fn insert_spare(
    &mut self,
    insert_at: Location,
    rotation: Rotation,
  ) -> Result<(), LocationError> {
    match insert_at {
      Location(x, 0) if x % 2 == 1 => self.rotate_down(x, rotation),
      Location(6, y) if y % 2 == 1 => self.rotate_left(y, rotation),
      Location(x, 6) if x % 2 == 1 => self.rotate_up(x, rotation),
      Location(0, y) if y % 2 == 1 => self.rotate_right(y, rotation),
      _ => Err(LocationError::new(&format!(
        "Cannot insert a tile at location {}",
        insert_at,
      ))),
    }
  }
}

struct BoardGraph {
  components: HashMap<Location, i32>,
}

impl BoardGraph {
  pub fn empty() -> BoardGraph {
    BoardGraph {
      components: HashMap::new(),
    }
  }
}

impl From<&Board> for BoardGraph {
  fn from(board: &Board) -> BoardGraph {
    /// Depth first search based component labeling utility function
    fn dfs_util(
      board: &Board,
      at: &Location,
      visited: &mut HashMap<Location, bool>,
      graph: &mut BoardGraph,
      component_id: i32,
    ) {
      if !visited.get(at).unwrap_or(&false) {
        visited.insert(*at, true);
        graph.components.insert(*at, component_id);
        for neighbor_at in board.neighbors(at).unwrap() {
          dfs_util(board, &neighbor_at, visited, graph, component_id);
        }
      }
    }

    let mut graph = BoardGraph {
      components: HashMap::new(),
    };

    let mut current_component_id = -1;
    let locations: Vec<Location> = board.placed.iter().map(|(location, _)| *location).collect();

    // Initially all locations are unvisited
    let mut visited: HashMap<Location, bool> = locations
      .iter()
      .map(|location| (*location, false))
      .collect();

    for location in locations {
      if !visited.get(&location).unwrap() {
        current_component_id += 1;
        dfs_util(
          board,
          &location,
          &mut visited,
          &mut graph,
          current_component_id,
        )
      }
    }

    graph
  }
}

impl BoardGraph {
  pub fn is_connected(
    &self,
    location1: &Location,
    location2: &Location,
  ) -> Result<bool, LocationError> {
    Ok(
      self
        .components
        .get(location1)
        .ok_or(LocationError::from(location1))?
        == self
          .components
          .get(location2)
          .ok_or(LocationError::from(location2))?,
    )
  }
}
