use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Copy, Clone)]
enum Item {
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

#[derive(Copy, Clone)]
enum Player {
    Player1,
    Player2,
    Player3,
    Player4,
}

#[derive(Copy, Clone)]
enum TileMarking {
    Item(Item),
    PlayerStart(Player),
}

#[derive(Copy, Clone)]
struct Tile {
    marking: Option<TileMarking>,
    path_up: bool,
    path_right: bool,
    path_down: bool,
    path_left: bool,
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
enum Rotation {
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

#[derive(Copy, Clone)]
struct PlacedTile(Tile, Rotation);

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
struct Location(usize, usize);

/// A board containing all tiles placed on the board and the spare extra tile
struct Board(HashMap<Location, PlacedTile>, Tile);

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

    /// Create a new board including the fixed tiles, with free tiles placed using the random number generator
    pub fn new<R: Rng>(rng: &mut R) -> Board {
        let fixed_tiles = Board::FIXED_TILES
            .clone()
            .map(|(location, tile)| (location, PlacedTile(tile, Rotation::Zero)));

        let mut free_tiles: Vec<PlacedTile> = Board::FREE_TILES
            .clone()
            .into_iter()
            .map(|tile| PlacedTile(tile, rng.gen()))
            .collect();

        let mut free_locations: Vec<Location> = Board::locations()
            .filter(|location| !fixed_tiles.map(|(location, _)| location).contains(location))
            .collect();

        // There should always be 1 more free tile than free location
        assert_eq!(free_locations.len(), free_tiles.len() - 1);

        free_tiles.shuffle(rng);
        free_locations.shuffle(rng);

        let extra_tile = free_tiles.pop().unwrap().0;
        let placed_tiles = free_locations.into_iter().zip(free_tiles);

        Board(
            fixed_tiles.into_iter().chain(placed_tiles).collect(),
            extra_tile,
        )
    }
}

fn main() {
    println!("Hello, world!");
}
