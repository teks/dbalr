/*
Floor Generation in Rogue
=========================
a new floor is done via new_level.c::new_level(). It generates everything from
the room layout to the gold and monster generation. In particular new_level() calls:
    * rooms.c::do_rooms(): rooms, gold, and monsters
    * do_passages()
    * there's no_food++ for some reason
    * new_level.c::put_things()

0 to 3 cells, chosen randomly, have no rooms ("ISGONE"; also empty cells may have passages?)

Room Generation - rooms.c::do_rooms():
---------------
rooms each go in one of 9 cells in a 3 x 3 grid.
room max size is 8 rows x 26 cols
up to 12 exits for some reason

setting room type:
    p(room is dark) = p(roll(0,9) < dungeon_level - 1)
    if a room is dark it may be a maze; p(dark room is a maze) = 1 in 15

mazes are about maximum size, ie cell size
non-maze rooms have minimum size about 4x4, max size is near the cell size
    so 4x4 (16 squares) to about 8x26 (208 squares)
    split into 3 categories:
    (16, 80), (81, 145), (146, 210)
    4x4 - 8x10
*/

#![allow(dead_code)] // not everything is implemented perfectly right away, rust, geez
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::random;
use std::collections::HashSet;
use std::collections::HashMap;

// Make an enum in rust actually useful, eg be HashMap & HashSet keys
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
enum CellLocation {
    NW, N, NE,
    W,  C, E,
    SW, S, SE,
}

impl CellLocation {
    // rust can't iterate over an enum's variants without a crate, so have to
    // resort to this brittle foolishness instead
    fn all() -> [CellLocation; 9] {
        use CellLocation::*;
        // formatting isn't so great because vscode can't format a selection
        [
            NW, N, NE,
            W,  C, E,
            SW, S, SE,
        ]
    }

    fn neighbors(&self) -> HashSet<CellLocation> {
        use CellLocation::*;
        match self {
            NW => HashSet::from([N, W]),
            NE => HashSet::from([N, E]),
            SE => HashSet::from([S, E]),
            SW => HashSet::from([S, W]),
            N  => HashSet::from([NW, C, NE]),
            E  => HashSet::from([NE, C, SE]),
            W  => HashSet::from([NW, C, SW]),
            S  => HashSet::from([SW, C, SE]),
            C  => HashSet::from([N, E, S, W]),
            // _  => no need for this since rust detects it's exhaustive
        }
    }
}

// Monster and Item are temporary placeholders; will be moved/implemented later
#[derive(Debug)]
struct Monster {}

#[derive(Debug)]
struct Item {}

// holds the data for a room regardless of room size.
// Passageways/connections are managed in the Floor object.
#[derive(Debug)]
struct Room {
    lit: bool,
    monsters: Vec<Monster>,
    items: Vec<Item>,
    stairs_up: bool,
    stairs_down: bool,
}

impl Room {
    fn new(lit: bool) -> Room {
        Room {
            lit,
            monsters: Vec::new(), // Vec::new() seems to infer that it should return a Vec<Monster>
            items: Vec::new(),
            stairs_up: false,
            stairs_down: false,
        }
    }
}

// enums can't be compared using `==` and `!=` unless you add PartialEq,
// however somehow they can be pattern-matched.
#[derive(Debug)]
enum Cell {
    SmallRoom(Room),
    MediumRoom(Room),
    LargeRoom(Room),
    Maze,
    Empty
}

// pass in die roll functions for ease of testing
type DieRoller = fn() -> u8; // "A DieRoller is a function that returns a u8 integer"

// From the cell locations, return N chosen at random, non-repeating
type RandomCellLocations = fn(u8) -> Vec<CellLocation>;

// p(room is dark) = p(roll(0,9) < dungeon_level - 1)
fn is_lit(dungeon_level: u8, lighting_d10: DieRoller) -> bool {
    lighting_d10() >= dungeon_level
}

// if a room is dark it may be a maze; p(dark room is a maze) = 1 in 15
fn is_maze(maze_d15: DieRoller) -> bool {
    maze_d15() == 15
}

/*
fn random_empty_cells() -> Vec<CellLocation> {
    use rand::prelude::*;
    let mut rng = rand::rng();
    let empty_cell_count = d4() - 1; // 0 - 3 empty cells in each dungeon level
    let all_loc = Vec::from(CellLocation::all());
}
*/

impl Cell {
    fn new_room(lit: bool, dungeon_level: u8, size_d3: DieRoller) -> Cell {
        // even chance it's a small, medium, or large room
        let room = match size_d3() {
            1 => Cell::SmallRoom,
            2 => Cell::MediumRoom,
            3 => Cell::LargeRoom,
            _ => panic!("Unreachable _ stanza reached"), // rust can't tell the die roll is exhaustive
        };
        room(Room::new(lit))
    }

    fn new(dungeon_level: u8,
        lighting_d10: DieRoller, is_maze_d15: DieRoller, room_size_d3: DieRoller,
    ) -> Cell {
        let is_lit = is_lit(dungeon_level, lighting_d10);
        let is_maze = !is_lit && is_maze(is_maze_d15);
        if is_maze { Cell::Maze }
        else { Cell::new_room(is_lit, dungeon_level, room_size_d3) }
    }
}

struct Floor {
    level: u8, // in rogue, AMULETLEVEL is 26 so 255 levels is plenty
    // hero_cell: CellLocation, // TODO the hero may not be on this floor;
    //      maybe a good time to use a rustism like an Option or such
    /* TODO checking for cells:
    [ ] always 9 Cells
    [ ] keyed by 1 of each CellLocation
    */
    cells: HashMap<CellLocation, Cell>,
    passages: HashSet<(CellLocation, CellLocation)>,
}

impl Floor {
    /// For consistent hashing, always orders the locations (greater, lesser).
    fn passage_order(a: CellLocation, b: CellLocation) -> (CellLocation, CellLocation) {
        if      a > b { (a, b) }
        else if a < b { (b, a) }
        else          { panic!("Passages cannot begin and end in the same cell.") }
    }

    /// Connect the two given cells with a passage.
    fn connect(&mut self, a: CellLocation, b: CellLocation) {
        if !a.neighbors().contains(&b) {
            panic!("{:?} are not neighbors; invalid passage", (a, b));
        }
        let passage = Self::passage_order(a, b);
        if self.passages.contains(&passage) {
            panic!("Floor already has passage {:?}", passage);
        }
        self.passages.insert(passage);
    }

    /// Passage Generation - passages.c::do_passages()
    fn do_passages() -> HashSet<(CellLocation, CellLocation)> {
        let mut passages = HashSet::<(CellLocation, CellLocation)>::new();
        // The graph of random corridors is always complete; one can reach any cell
        // from any other cell (but some doors are hidden). Randomly a few extra
        // corridors are added so that there can be loops.

        // Two adjacent cells can have only zero or one corridor between them.

        // Any corridors going into an empty cell will connect to each other.
        // That way one can walk from a room, through an empty cell, to another room.
        // If only one corridor enters an empty cell, however, it is a dead end.

        // The algorithm to build the initial graph is basically to pick a starting cell,
        // then drunkard's walk around the cells until they're all reachable.

        // Then, to add extra corridors, loop (roll_die(0, 4) times:
        //     1. pick a random cell
        //     2. find a random adjacent room that isn't already connected
        //     3. connect the two cells
        passages
    }

    fn new(level: u8, empty_cell_count_d4: DieRoller) -> Floor {
        let mut cells: HashMap<CellLocation, Cell> = HashMap::new();
        let mut candidate_locations = Vec::from(CellLocation::all());

        // randomly pick the empty cells
        let empty_cnt = empty_cell_count_d4() - 1;
        for i in 0..empty_cnt {
            let l = candidate_locations.len();
            let empty_location= candidate_locations.remove(rand::random_range(0..l));
            // print!("Empty cell #{}: {:?}\n", i, empty_location);
            cells.insert(empty_location, Cell::Empty);
        }

        // generate the non-empty cells
        for location in candidate_locations {
            let cell = Cell::new(level,
                || random::roll_die(1, 10),
                || random::roll_die(1, 15),
                || random::roll_die(1, 3), );
            // print!("Non-empty location: {:?} {:?}\n", location, cell);
            cells.insert(location, cell);
        }

        Floor { level, cells, passages: Self::do_passages(), }
    }

}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)] // I know how to name things thanks
    use super::*;

    #[test]
    fn CellLocation_neighbors() {
        // test a few cases:
        let nw_neighbors = CellLocation::NW.neighbors();
        assert_eq!(nw_neighbors, HashSet::from([CellLocation::N, CellLocation::W]));

        let c_neighbors = CellLocation::C.neighbors();
        assert_eq!(c_neighbors, HashSet::from([CellLocation::N, CellLocation::E, CellLocation::S, CellLocation::W]));

        let s_neighbors = CellLocation::S.neighbors();
        assert_eq!(s_neighbors, HashSet::from([CellLocation::SW, CellLocation::C, CellLocation::SE]));
    }

    #[test]
    fn Cell_new_room() {
        fn deterministic_die_fn() -> u8 { 1 }
        let c: Cell = Cell::new_room(true, 1, deterministic_die_fn);
        // assert_matches is evidently unstable // assert!(matches!(c, Cell::SmallRoom(_)));
        // apparently the only ways to extract data from an enum variant is `match` and `if let`
        let lit = match c {
            Cell::SmallRoom(room) => room.lit,
            _ => panic!("Expected Cell::SmallRoom, got {:?}", c),
        };
        assert!(lit);
    }

    #[test]
    fn Cell_new() {
        let dungeon_level = 5;
        fn three() -> u8 { 3 }
        fn seven() -> u8 { 7 }
        fn fifteen() -> u8 { 15 }

        // It'd be better test these independently, more like parametrizing a
        // test in pytest; unsure how best to do that in rust

        // lit room: d10 >= dungeon lvl
        let c = Cell::new(dungeon_level, seven, fifteen, three);
        if let Cell::LargeRoom(r) = c { assert!(r.lit) }
        else { panic!("Expected LargeRoom, got {:?}", c) }

        // dark room: d10 < dungeon lvl && d15 != 15
        let c = Cell::new(dungeon_level, three, seven, three);
        if let Cell::LargeRoom(r) = c { assert!(!r.lit) }
        else { panic!("Expected LargeRoom, got {:?}", c) }

        // dark maze (all mazes are dark): d10 < dungeon lvl && d15 == 15
        let c = Cell::new(dungeon_level, three, fifteen, three);
        assert!(matches!(c, Cell::Maze));
    }

    #[test]
    fn Floor_new() {
        use super::*;
        fn three() -> u8 { 3 }
        let floor = Floor::new(13, three);
        assert_eq!(floor.level, 13);
        assert_eq!(floor.cells.len(), 9);
        let mut empty_cnt = 0;
        for (location, cell) in floor.cells {
            match cell {
                Cell::Empty => empty_cnt += 1,
                _ => (),
            }
        }
        assert_eq!(empty_cnt, 2);
    }

    #[test]
    fn Floor_passage_order() {
        let po_tuple = Floor::passage_order(CellLocation::NW, CellLocation::C);
        assert_eq!((CellLocation::C, CellLocation::NW), po_tuple);
        let po_tuple = Floor::passage_order(CellLocation::S, CellLocation::SW);
        assert_eq!((CellLocation::S, CellLocation::SW), po_tuple);
    }

    #[test]
    #[should_panic]
    fn Floor_passage_order_panic() {
        Floor::passage_order(CellLocation::C, CellLocation::C);
    }

    #[test]
    fn Floor_connect() {
        let mut floor = Floor::new(2, || 3);
        floor.connect(CellLocation::N, CellLocation::NW);
        floor.connect(CellLocation::SE, CellLocation::S);
        assert_eq!(floor.passages.len(), 2);
    }

    /// Floor::connect should panic when two cells are already connected
    #[test]
    #[should_panic]
    fn Floor_connect_dupe_panic() {
        let mut floor = Floor::new(2, || 3);
        floor.connect(CellLocation::C, CellLocation::S);
        floor.connect(CellLocation::C, CellLocation::S);
    }

    /// Floor::connect should panic when the two cells are not adjacent
    #[test]
    #[should_panic]
    fn Floor_connect_nonadjacent_panic() {
        let mut floor = Floor::new(2, || 3);
        floor.connect(CellLocation::E, CellLocation::W);
    }
}
