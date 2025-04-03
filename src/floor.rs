/*

Most of rogue's randomness seems to be even flat probabilities, ie rnd(10) is 1d10 - 1.

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

Passage Generation
------------------
TODO NEXT!
*/

#![allow(dead_code)] // not everything is implemented perfectly right away, rust, geez
#![allow(unused_variables)]

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)] // without this line, CellLocations can't be HashMap keys
enum CellLocation {
    NW, N, NE,
    W,  C, E,
    SW, S, SE,
}

impl CellLocation {
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

// p(room is dark) = p(roll(0,9) < dungeon_level - 1)
fn is_lit(dungeon_level: u8, lighting_d10: DieRoller) -> bool {
    lighting_d10() >= dungeon_level
}

// if a room is dark it may be a maze; p(dark room is a maze) = 1 in 15
fn is_maze(maze_d15: DieRoller) -> bool {
    maze_d15() == 15
}

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
    // TODO empty: 0 - 3 empty cells in each dungeon level
    level: u8, // in rogue, AMULETLEVEL is 26 so 255 levels is plenty
    hero_cell: CellLocation,
    /* TODO checking for cells:
    [ ] always 9 Cells
    [ ] keyed by 1 of each CellLocation
    */
    cells: HashMap<CellLocation, Cell>,
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)] // I know how to name things thanks
    use super::*;

    #[test]
    fn test_CellLocation_neighbors() {
        // test a few cases:
        let nw_neighbors = CellLocation::NW.neighbors();
        assert_eq!(nw_neighbors, HashSet::from([CellLocation::N, CellLocation::W]));

        let c_neighbors = CellLocation::C.neighbors();
        assert_eq!(c_neighbors, HashSet::from([CellLocation::N, CellLocation::E, CellLocation::S, CellLocation::W]));

        let s_neighbors = CellLocation::S.neighbors();
        assert_eq!(s_neighbors, HashSet::from([CellLocation::SW, CellLocation::C, CellLocation::SE]));
    }

    #[test]
    fn test_Cell_new_room() {
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
    fn test_Cell_new() {
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
}
