/* Floor objects have 9 cells in a 3 x 3 grid.
*/

#![allow(dead_code)] // not everything is implemented perfectly right away, rust, geez

use std::collections::HashSet;
use std::collections::HashMap;

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
    cell_type: CellType, // TODO enum or subclass? (TODO can structs be 'subclassed?')
    /* TODO possible contents:
        @: 0 or 1
        monsters: 0+
        treasures/items: 0+
        stairs up:   0 or 1
        stairs down: 0 or 1
    */
    // TODO OR should I put the contents in the floor object then give each obj a CellLocation instead?
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

}
