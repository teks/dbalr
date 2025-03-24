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

effects of room size:
* larger rooms delay onset of melee more (enemies need time to reach you)
* larger rooms may have more spawns (check rogue source):
    * monsters
    * items
    * traps
    * stairs

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

