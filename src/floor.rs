/* Floor objects have 9 cells in a 3 x 3 grid.
*/

enum CellType {
    SmallRoom,
    MediumRoom,
    LargeRoom,
    Maze,
    Empty
}

enum CellLocation {
    NW, N, NE,
    W,  C, E,
    SW, S, SE,
}

impl CellLocation {
    fn neighbors(&self) {
    }
}

struct Floor {

}
