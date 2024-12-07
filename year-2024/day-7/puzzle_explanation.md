Day 7: Bridge Repair
===

- https://adventofcode.com/2024/day/7

The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief isn't on this side of the bridge, though; maybe he's on the other side?

When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.

You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some young elephants were playing nearby and <span class="aoc-glow">stole all the operators</span> from their calibration equations! They could finish the calibrations if only someone could determine which test values could possibly be produced by placing any combination of operators into their calibration equations (your puzzle input).

For example:
```plaintext
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

Each line represents a single equation. The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.

Operators are <span class="aoc-glow">always evaluated left-to-right, not</span> according to precedence rules. Furthermore, numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants holding two different types of operators: <span class="aoc-glow">add</span> (`+`) and <span class="aoc-glow">multiply</span> (`*`).

Only three of the above equations can be made true by inserting operators:

- `190: 10 19` has only one position that accepts an operator: between `10` and `19`. Choosing `+` would give `29`, but choosing `*` would give the test value (`10 * 19 = 190`).
- `3267: 81 40 27` has two positions for operators. Of the four possible configurations of the operators, <span class="aoc-glow">two</span> cause the right side to match the test value: `81 + 40 * 27` and `81 * 40 + 27` both equal `3267` (when evaluated left-to-right)!
- `292: 11 6 16 20` can be solved in exactly one way: `11 + 6 * 16 + 20`.

The engineers just need the <span class="aoc-glow">total calibration result</span>, which is the sum of the test values from just the equations that could possibly be true. In the above example, the sum of the test values for the three equations listed above is <span class="aoc-glow">`3749`</span>.

Determine which equations could possibly be true. <span class="aoc-glow">What is their total calibration result?</span>

---

[My Solution](https://github.com/antonio-hickey/advent-of-code/blob/master/year-2024/day-7/src/main.rs)

To parse the puzzle data into a meaningful model, I want to extract the target result and all the equation parameters I can add or multiply to get the target result.

```rust
#[derive(Debug, Default)]
/// A model of the puzzle input data
struct Calibration {
    target: i64,
    parameters: Vec<i32>,
}
```

I do this by going over each line in the data, splitting it first by ":" which gives me 2 sections (target result, and parameters). Finally split the parameters string by " " and collect into a vector of integers.

```rust
impl Calibration {
    /// Parse the puzzle input data into a meaningful model
    fn from_puzzle_input(puzzle_data: &str) -> Vec<Self> {
        // Fold each line in the data into a instance of `Calibration`
        // and collect each instance into a `Vec<Calibration>`
        puzzle_data
            .lines()
            .fold(Vec::new(), |mut calibrations, line| {
                // ex: "190: 10 19" = ["190", " 10 19"]
                let mut sections = line.split(":");

                // parse the target result into an integer
                let target: i64 = sections
                    .next()
                    .expect("AoC not to have bad data")
                    .parse()
                    .expect("AoC not to have bad data");

                // parse the parameters into a vector of integers
                // (have to trim the first " " and then split by " ")
                let parameters: Vec<i32> = sections
                    .next()
                    .expect("AoC not to have bad data")
                    .trim_start()
                    .split(" ")
                    .map(|param| param.trim().parse().expect("AoC not to have bad data"))
                    .collect();

                // Collect parsed results into the model
                calibrations.push(Calibration { target, parameters });
                calibrations
            })
    }
}
```

After parsing the data into my model, I iterate over each calibration and filter out 
invalid calibrations, finally summing up all the valid calibration target results.

```rust
calibrations
    .iter()
    .filter(|calibration| is_valid_calibration(calibration))
    .map(|calibration| calibration.target)
    .sum()
```

The tricky part is determining what values are valid or invalid. You need to try $<span data-tex>2^{n - 1}$</span> possible equations, where <span data-tex>$n$</span> is the number of parameters in the equation. 

So [`81, 40, 27`] for example would have <span data-tex>$2^{3 - 1 = 2} = 4$</span> possible equations:
- <span data-tex>81 + 40 + 27</span>
- <span data-tex>81 * 40 + 27</span>
- <span data-tex>81 * 40 * 27</span>
- <span data-tex>81 + 40 * 27</span>

From here I keep track of 2 different variables, the current result of the equation and the
remaining parameters in the equation. Each iteration I compute both the current result + the next parameter and the current result * the next parameter. Finally I check all the possible equation results to see if the target result is contained.
```rust
/// Determine if a `Calibration` is valid.
fn is_valid_calibration(calibration: &Calibration) -> bool {
    let mut is_valid_calibration = false;
    let params = &calibration.parameters;

    let mut equation_results = vec![params[0] as i64];

    // go through each parameter in the equation
    params[1..].iter().for_each(|&param| {
        let mut next_results = Vec::new();

        // check both operations for each equation result
        equation_results.iter().for_each(|result| {
            next_results.push(result + param as i64);
            next_results.push(result * param as i64);
        });

        // update all the different equation results
        equation_results = next_results;
    });

    // check if any of the equation results
    // are the calibrations target result
    if equation_results.contains(&calibration.target) {
        is_valid_calibration = true;
    }

    is_valid_calibration
}
```

