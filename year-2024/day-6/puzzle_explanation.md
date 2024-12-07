Day 6: Guard Gallivant
===

The Historians use their fancy device again, this time to whisk you all away to the North Pole prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while The Historians search for the Chief. Unfortunately, a single <span class="aoc-glow">guard</span> is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?

You start by making a map (your puzzle input) of the situation. For example:

```plaintext
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

The map shows the current position of the guard with `^` (to indicate the guard is currently facing **up** from the perspective of the map). Any **obstructions** - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

- If there is something directly in front of you, turn right 90 degrees.
- Otherwise, take a step forward.

Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):

```
....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
```

Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:

```
....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...
```

Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...
```

This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..
```

By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. **Including the guard's starting position**, the positions visited by the guard before leaving the area are marked with an `X`:

```
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..
```

In this example, the guard will visit **`41`** distinct positions on your map.

Predict the path of the guard. **How many distinct positions will the guard visit before leaving the mapped area?**

- [My Solution](https://github.com/antonio-hickey/advent-of-code/blob/f705efbccbebc277f9bab03fe87e7cda8652c73c/year-2024/day-6/src/main.rs#L19-L65) 
    The parsing is pretty simple. I loop over every line, and every character of text in the puzzle data
    forming a grid of boolean values (false). I also check if the current character is an obstruction and if
    so I collect it's position in the grid. The last piece of information I parse out of the puzzle data is the guards starting position denoted by `^`.
    ```rust
    #[derive(Debug, Default)]
    /// A model of the puzzle input data
    struct GuardMap {
        grid: Vec<Vec<bool>>,
        obstructions: Vec<(i32, i32)>,
        starting_position: (i32, i32),
    }
    impl GuardMap {
        /// Parse the puzzle input data into a meaningful model
        fn from_puzzle_input(puzzle_data: &str) -> Self {
            puzzle_data.lines().enumerate().fold(
                GuardMap::default(),
                |mut guard_map, (line_idx, line)| {
                    guard_map.grid.push(Vec::new());
    
                    line.chars().enumerate().for_each(|(char_idx, char)| {
                        guard_map.grid[line_idx].push(false);
    
                        if char == '#' {
                            guard_map
                                .obstructions
                                .push((line_idx as i32, char_idx as i32));
                        } else if char == '^' {
                            guard_map.starting_position = (line_idx as i32, char_idx as i32);
                            guard_map.grid[line_idx][char_idx] = true;
                        }
                    });
    
                    guard_map
                },
            )
        }
    } 
    ```

    --

    The solution is simple as well, actually got it first try which rarely happens lol. I take an iterative approach and keep moving the guard's current position based on their current direction. To move up in the grid you just need to subtract the current x coordinate by 1, to move down you add the current x coordinate by 1, to move left and right you add/subtract 1 from the y coordinate. 

    Every time I move the guard I mutate the grid value to be true, marking that they've been there. I do this until the guard's next location would be an obstruction which can't happen, so then I change the direction by 90 degrees and restart. 

    Once the guard hits the edge of the grid I stop moving them, and filter the grid for all the true values (positions they've been) and get the count as my answer.

    ```rust
    // The guards current location and direction
    let mut guards_current_position = guard_map.starting_position;
    let mut guards_current_direction = '^';
    let directions = ['^', '>', 'v', '<'];

    loop {
        // Calculate the guards next location
        let mut direction_idx = directions
            .iter()
            .position(|&d| d == guards_current_direction)
            .expect("Guard to always have a valid direction");

        let next_coordinate = match guards_current_direction {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Invalid direction"),
        };

        let next_location = (
            guards_current_position.0 + next_coordinate.0,
            guards_current_position.1 + next_coordinate.1,
        );

        // Check if the next location is an obstruction
        // and handle it with a direction change or move
        // the guard to the next location
        if guard_map.obstructions.contains(&next_location) {
            direction_idx = (direction_idx + 1) % directions.len();
            guards_current_direction = directions[direction_idx];
            continue;
        } else {
            guard_map.grid[next_location.0 as usize][next_location.1 as usize] = true;
            guards_current_position = next_location;
        }

        // Check if the guard has reached the end of the grid
        if match guards_current_direction {
            '<' => guards_current_position.1 == 0,
            '>' => guards_current_position.1 == (guard_map.grid[0].len() - 1) as i32,
            'v' => guards_current_position.0 == (guard_map.grid.len() - 1) as i32,
            '^' => guards_current_position.0 == 0,
            _ => false,
        } {
            break;
        }
    }

    // Get the count of position's the guard went to
    guard_map.grid.iter().fold(i32::default(), |acc, row| {
        acc + row.iter().filter(|&&col| col).count() as i32
    })

    ```
