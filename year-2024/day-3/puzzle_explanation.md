Day 3: Mull It Over
===

- https://adventofcode.com/2024/day/3

"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is **corrupted**. All of the instructions have been jumbled up!

It seems like the goal of the program is just to **multiply some numbers**. It does that with instructions like ```mul(X,Y)```, where `X` and `Y` are each 1-3 digit numbers. For instance, ```mul(44,46)``` multiplies `44` by `46` to get a result of `2024`. Similarly, ```mul(123,4)``` would multiply `123` by `4`.

However, because the program's memory has been corrupted, there are also many invalid characters that should be **ignored**, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do **nothing**.

For example, consider the following section of corrupted memory:

```x**mul(2,4)**%&mul[3,7]!@^do_not_**mul(5,5)**+mul(32,64]then(**mul(11,8)****mul(8,5)**)```

Only the four highlighted sections are real `mul` instructions. Adding up the result of each instruction produces **161** (`2*4 + 5*5 + 11*8 + 8*5`).

Scan the corrupted memory for uncorrupted `mul` instructions. **What do you get if you add up all of the results of the multiplications?**

- [My Solution](https://github.com/antonio-hickey/advent-of-code/blob/29cae2359b9ca0946e230b1790989f67a63600db/year-2024/day-3/src/main.rs#L13-L83), This one is really just a parsing puzzle 😒. I loop through the corrupted memory collecting mul instructions as long as they fit certain conditions. First I go through each occurrence of "mul" and check the size of it. The max parameter size is 3 digits so 999 meaning the max length of a valid mul instruction is 12 `mul(999,999)` = 12 chars and the min length is `mul(1,1)` = 8 chars. Then I grab the parameter space of the instruction (the digits in between "(" and ")" ex: `"999,999"`), and split that section by `","` and try to parse them into integers as if they don't parse then it's not a valid instruction. When finished I have a vector of all the valid mul op's and can just compute the product of each one and then sum up all those products for an answer to the puzzle.
