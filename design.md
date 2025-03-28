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

## Room Sizes in Rogue
In rogue, minimum room size is about 4x4, max size is near the cell size
(8x26).

The player always appears along an edge (a door), while the monster placed in a
room can be anywhere.  This means the starting encounter distance is always
from 1 to 26 (the max length of a room's longest axis). Diagonal movement is 1
turn just as cardinal movement is, meaning many placements have equivalent
distance.

However, the player may be entering from the E or W, meaning the likely shorter
vertical edge, or N or S, meaning the likely longer horizontal edge. Entry from
N or S is thus more likely to have shorter encounter distances.

I don't think it's necessary to represent these probabilities in dbalr exactly.
The main thing to capture is:
* larger rooms delay onset of melee more (enemies need time to reach you):
    * small, medium, large => 0, 1, 2 turns
* A random factor will vary the encounter distance, to represent how random
  placement of monster & doors affects such distances a lot.
* larger rooms may have more spawns (check rogue source):
    * monsters - so far when rooms are first created, only 0 or 1 monster
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

