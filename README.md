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

* The LLM decided to provide a debug function that would print our working directory, even though
  I didn't explicitly asked it to. This was good though, because I was working on trying to
  fix a path bug. Interesting that it chose to add this to the code.
* Exporting data from Claude is very annoying, will need to revisit this.