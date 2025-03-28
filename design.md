# SCALE
The classical roguelikes are at a scale of small squares, 5 or 10 ft wide.
Dbalr is at 'room scale,' where 'one square' equals an entire room that could
be dozens of feet long.

# DUNGEON LAYOUT
each dungeon level has 3 x 3 cells; basically copy rogue's algorithms

each cell can be one of:
* room: small, medium, or large
* maze
* empty

## Room Sizes vs. Rogue
In rogue, minimum size about 4x4, max size is near the cell size (8x26).
This is 16 squares to 208 squares. Split into 3 categories:
* small:  16 to 80 squares,   about 4x4  to 6x12
* medium: 81 to 145 squares,  about 6x13 to 7x14
* large:  146 to 210 squares, about 7x14 to 8x26

```
............
............
............
............ 6x12 (biggest small)
............
............

..........................
..........................
..........................
..........................
.......................... 8x26 (biggest big)
..........................
..........................
..........................
```

On average, monster will appear in the center.
Player always appears along an edge (a door).
I believe diagonal movement is 1 turn, same as cardinal movement.
So, how many turns on average does an archer have before the monster closes to melee?

[ ] Calculate this ---^

effects of room size:
* larger rooms delay onset of melee more (enemies need time to reach you):
    * small, medium, large => 1, 2, 3 turns
* larger rooms may have more spawns (check rogue source):
    * monsters
    * items
    * traps
    * stairs

## Mazes, Hallways, and Stairs

effects of mazes:
* need time to explore it to find any spawns or exits
* enemies are encountered at melee range (due to lack of light)

hallways
* no spawns in hallways
* all the rooms in a level must be connected by hallways
* a hallway may end in an empty cell as a dead end
* hallways can also pass through empty cells to reach a room

stairs
* for now, copy rogue: you can't go back up
* later, have a setting for persistent dungeon levels

