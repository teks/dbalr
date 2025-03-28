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
    SO even small 

Passage Generation
------------------
TODO NEXT!
*/

#![allow(dead_code)] // not everything is implemented perfectly right away, rust, geez
#![allow(unused_variables)]

use std::collections::HashSet;
use std::collections::HashMap;

// enums can't be compared using `==` and `!=` unless you add PartialEq,
// however somehow they can be pattern-matched.
#[derive(Debug, PartialEq)]
enum CellType {
    SmallRoom,
    MediumRoom,
    LargeRoom,
    Maze,
    Empty
}

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

struct Cell {
    cell_type: CellType, // TODO now leaning towards Cell being an enum-with-data
    lit: bool,
    /* TODO possible contents:
        @: 0 or 1
        monsters: 0+
        treasures/items: 0+
        stairs up:   0 or 1
        stairs down: 0 or 1
        passageways/connections? <-- TODO
    */
    // TODO OR should I put the contents in the floor object then give each obj a CellLocation instead?
}

impl Cell {
    // pass in die roll function for ease of testing:
    fn new(level: u8, dieroll_fn: fn(u8) -> u8) -> Cell {
        let cell = Cell {
            lit: false, // TODO p(room is dark) = p(roll(0,9) < dungeon_level - 1)
            // TODO if a room is dark it may be a maze; p(dark room is a maze) = 1 in 15
            // even chance it's a small, medium, or large room
            cell_type: match dieroll_fn(3) {
                1 => CellType::SmallRoom,
                2 => CellType::MediumRoom,
                3 => CellType::LargeRoom,
                _ => panic!("Unreachable _ clause in match"), // rust can't tell the die roll is exhaustive
            },
        };
        cell
    }
}

struct Floor {
    level: u8, // in rogue, AMULETLEVEL is 26 so 255 levels is plenty
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
    fn test_Cell_new() {
        fn deterministic_die_fn(sides: u8) -> u8 { 1 }
        let c = Cell::new(255, deterministic_die_fn);
        assert_eq!(c.cell_type, CellType::SmallRoom);
    }

}
