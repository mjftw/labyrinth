use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::collections::HashMap;

//TODO
#[derive(Copy, Clone)]
enum Item {
    Ruby,
    Sword,
    Bat,
    Mouse,
}

#[derive(Copy, Clone)]
struct Tile {
    item: Option<Item>,
    path_up: bool,
    path_right: bool,
    path_down: bool,
    path_left: bool,
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

impl PlacedTile {
    /// Rotate the placed tile clockwise by 90 degrees
    pub fn rotate_cw(&mut self) {
        self.1 = match self.1 {
            Rotation::Zero => Rotation::Clockwise90,
            Rotation::Clockwise90 => Rotation::Clockwise180,
            Rotation::Clockwise180 => Rotation::Clockwise270,
            Rotation::Clockwise270 => Rotation::Zero,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Location(usize, usize);

type BoardTiles = HashMap<Location, PlacedTile>;

/// A board containing all tiles placed on the board and the spare extra tile
struct Board(BoardTiles, Tile);

impl Board {
    //TODO:
    /// The tiles that are fixed to the board and cannot be moved or rotated
    const FIXED_TILES: [(Location, Tile); 16] = [(
        Location(0, 0),
        Tile {
            item: None,
            path_up: false,
            path_right: false,
            path_down: false,
            path_left: false,
        },
    ); 16];

    //TODO:
    /// The tiles that are free to be placed or rotated
    const FREE_TILES: [Tile; 33] = [Tile {
        item: None,
        path_up: true,
        path_down: true,
        path_left: true,
        path_right: true,
    }; 33];

    const INSERT_LOCATIONS: [Location; 9] = [
        Location(1, 1),
        Location(1, 3),
        Location(1, 5),
        Location(3, 1),
        Location(3, 3),
        Location(3, 5),
        Location(5, 1),
        Location(5, 3),
        Location(5, 5),
    ];

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

        let mut free_locations: Vec<Location> = (0..7)
            .permutations(2)
            .filter_map(|xy| match &xy[..] {
                &[x, y]
                    if fixed_tiles
                        .map(|(location, _)| location)
                        .contains(&Location(x, y)) =>
                {
                    None
                }
                &[x, y] => Some(Location(x, y)),
                _ => None,
            })
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

    /// Try to insert a tile at a given location, sliding all the tiles in the row/column by 1.
    /// Inserting a tile pushes the tile opposite off the board.
    /// Returns Some(tile) with the pushed off tile if insertion was possible, and None if not.
    /// Valid insertion locations are (1,1), (1,3), (1,5), (3,1), (3,3), (3,5), (5,1), (5,3), (5,5)
    pub fn insert(&mut self, tile: &PlacedTile, location: &Location) -> Option<Tile> {
        if Board::INSERT_LOCATIONS.iter().contains(location) {
            None
        } else {
            None
        }
    }
}

//TODO: Add tests

fn main() {
    println!("Hello, world!");
}
