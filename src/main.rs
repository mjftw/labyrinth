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

    /// Rotate row y left, replacing the rightmost tile with the spare tile
    fn rotate_left(&mut self, y: usize, tile_rotation: Rotation) {
        let pushed_out = self.0.remove(&Location(0, y)).unwrap().0;

        for x in (1..7).rev() {
            let move_to = Location(x - 1, y);
            let move_from = Location(x, y);
            let moving_tile = self.0.remove(&move_from).unwrap();
            self.0.insert(move_to, moving_tile);
        }

        self.0
            .insert(Location(6, y), PlacedTile(self.1, tile_rotation));
        self.1 = pushed_out;
    }

    /// Rotate row y right, replacing the leftmost tile with the spare tile
    fn rotate_right(&mut self, y: usize, tile_rotation: Rotation) {
        let pushed_out = self.0.remove(&Location(6, y)).unwrap().0;

        for x in 1..7 {
            let move_to = Location(x, y);
            let move_from = Location(x - 1, y);
            let moving_tile = self.0.remove(&move_from).unwrap();
            self.0.insert(move_to, moving_tile);
        }

        self.0
            .insert(Location(0, y), PlacedTile(self.1, tile_rotation));
        self.1 = pushed_out;
    }

    /// Rotate column x up, replacing the bottommost tile with the spare tile
    fn rotate_up(&mut self, x: usize, tile_rotation: Rotation) {
        let pushed_out = self.0.remove(&Location(x, 0)).unwrap().0;

        for y in (1..7).rev() {
            let move_to = Location(x, y - 1);
            let move_from = Location(x, y);
            let moving_tile = self.0.remove(&move_from).unwrap();
            self.0.insert(move_to, moving_tile);
        }

        self.0
            .insert(Location(x, 6), PlacedTile(self.1, tile_rotation));
        self.1 = pushed_out;
    }

    /// Rotate column x down, replacing the topmost tile with the spare tile
    fn rotate_down(&mut self, x: usize, tile_rotation: Rotation) {
        let pushed_out = self.0.remove(&Location(x, 6)).unwrap().0;

        for y in 1..7 {
            let move_to = Location(x, y);
            let move_from = Location(x, y - 1);
            let moving_tile = self.0.remove(&move_from).unwrap();
            self.0.insert(move_to, moving_tile);
        }

        self.0
            .insert(Location(x, 0), PlacedTile(self.1, tile_rotation));
        self.1 = pushed_out;
    }

    /// Try to insert the extra tile at a given location, sliding all the tiles in the row/column by 1.
    /// Inserting a tile pushes the tile opposite off the board, which becomes the new extra tile.
    /// Returns Ok(()) if insertion was possible, and Err(()) if not.
    /// Valid insertion locations are (1,1), (1,3), (1,5), (3,1), (3,3), (3,5), (5,1), (5,3), (5,5)
    pub fn insert_spare(&mut self, insert_at: Location, rotation: Rotation) -> Result<(), ()> {
        match insert_at {
            Location(0, y) => {
                self.rotate_right(y, rotation);
                Ok(())
            }
            Location(6, y) => {
                self.rotate_left(y, rotation);
                Ok(())
            }
            Location(x, 0) => {
                self.rotate_down(x, rotation);
                Ok(())
            }
            Location(x, 6) => {
                self.rotate_up(x, rotation);
                Ok(())
            }
            _ => Err(()),
        }
    }
}

//TODO: Add tests

fn main() {
    println!("Hello, world!");
}
