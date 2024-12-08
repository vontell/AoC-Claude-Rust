# AoC-Claude-Rust

This project contains results from solving the [Advent of Code](https://adventofcode.com/2024)
puzzles for 2024 using only LLM-written code. My personal goals for this are to:

1. Learn the best ways to interact with LLMs to get them to write and iterate on code
2. See what the current pitfalls and limitations are for these LLMs
3. Learn Rust (which I have never used before)


**Note: These submissions will only be posted after the leaderboard has been filled, since
  using LLMs are against the rules.**

Here are the rules I am setting for myself:

* I can never write any code myself - all code must be written and copy-pasted from the LLMs
* I will use Claude chat for almost all of these submissions, although depending on how that
  goes, I may also consider using Cursor to edit files directly.
* The entire chat from the LLM will be exported and included in this project.

# How to run the solution for one of the days

To run the tests:

```bash
# Run tests just for day 1
cargo test --test day1
```

To run the main function:

```bash
cargo run --bin day1
```

# My Learnings

### Day 1

**Part 1**

* The LLM decided to provide a debug function that would print our working directory, even though
  I didn't explicitly asked it to. This was good though, because I was working on trying to
  fix a path bug. Interesting that it chose to add this to the code.
* Exporting data from Claude is very annoying, will need to revisit this.

**Part 2**

* Solved in one shot. The model elected to edit my original code rather than trying to resolve it. It did leave
  the answer for part 1 intact as well, which was nice.

### Day 2

**Part 1**

* Basically one shot to solve the first step, with a pretty simple prompt too:

```text
Solve the following program in rust. Expect an input text file with the reports to be located at puzzles/day2/input.txt. We will run the code through a main() function.
```

* Was much easier now that the project structure was setup. Note that I did modify the Cargo.toml file myself to support
  running this day's solution as a bin.

**Part 2**

* Solved it with the breeze. This time, unlike day 1, it did get rid of the part 1 solution partially.
* I find myself breezing through this so quickly that I haven't even had to think about the problem or read the code...

### Day 3

**Part 1**

* This is the first day when we relied on importing a third-party dependency. I gave the error to Claude and it
  resolved it easily, although it was interesting to see the version it selected for the package (obviously a tad
  outdated, but it was legit). 
* Answered in one shot once again.
* I explicitly asked today for it to include comments in the code. It did leave nice comments for all major parts of
  the code.

**Part 2**

* Once again, solved in one shot, and it chose to edit the original code and not leave the part 1 solution intact.

### Day 4

**Part 1**

* I felt like the logic for this one was the first one in the calendar that seemed a little trickier... but still a
  breeze, got it on the first try.