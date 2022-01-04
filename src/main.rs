use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::collections::HashMap;

//TODO
enum Item {
    Ruby,
    Sword,
    Bat,
    Mouse,
}

struct Tile {
    item: Option<Item>,
    path_up: bool,
    path_down: bool,
    path_left: bool,
    path_right: bool,
}

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

#[derive(Hash, PartialEq, Eq)]
struct Location(usize, usize);

type BoardTiles = HashMap<Location, PlacedTile>;

/// A board containing all tiles placed on the board and the spare extra tile
struct Board(BoardTiles, Tile);

impl Board {
    /// The tiles that are fixed to the board and cannot be moved or rotated
    fn fixed_tiles() -> BoardTiles {
        //TODO
        HashMap::new()
    }

    /// The tiles that are free to be placed or rotated
    fn free_tiles() -> Vec<Tile> {
        //TODO
        Vec::new()
    }

    /// Create a new board including the fixed tiles, with free tiles placed using the random number generator
    pub fn new<R: Rng>(rng: &mut R) -> Board {
        let mut free_tiles: Vec<PlacedTile> = Board::free_tiles()
            .into_iter()
            .map(|tile| PlacedTile(tile, rng.gen()))
            .collect();

        let mut free_locations: Vec<Location> = (0..7)
            .permutations(2)
            .map(|xy| match &xy[..] {
                &[x, y] => Location(x, y),
                _ => panic!("Error building free locations"),
            })
            .collect();

        // These should always be 1 more free tile than free locations
        assert_eq!(free_locations.len(), free_tiles.len() - 1);

        free_tiles.shuffle(rng);
        free_locations.shuffle(rng);

        let extra_tile = free_tiles.pop().unwrap().0;
        let placed_tiles = free_locations.into_iter().zip(free_tiles);

        Board(
            Board::fixed_tiles()
                .into_iter()
                .chain(placed_tiles)
                .collect(),
            extra_tile,
        )
    }

    /// Try to insert a tile at a given location.
    /// Inserting a tile pushes the tile opposite off the board.
    /// Returns Some(tile) with the pushed off tile if insertion was possible, and None if not.
    pub fn insert(&mut self, tile: &PlacedTile, location: &Location) -> Option<Tile> {
        //TODO
        None
    }
}

//TODO: Add tests

fn main() {
    println!("Hello, world!");
}
