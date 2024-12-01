# Advent of Code 2024

[Advent of Code Website](https://adventofcode.com/)

Scripts written to solve the 2024 edition of Advent of Code. This has become my yearly dabble into Rust. I feel I get 
up to speed quicker each year, and it is very hard to give up the speed that rust can solve what usually tend to be 
quite low-level challenges.

[`main.rs`](./src/main.rs) - This is the entry point to the script, and follows a pattern of asking for a day to run, 
then deferring to `day_X.rs` for each days' solutions. Unit tests for each day written based on the examples given in 
the puzzle descriptions are in a `tests` submodule in that day's file.

Alongside the puzzles I'm using GitHub actions / pages to automate publishing the docs, and I'm using 
[11ty](https://www.11ty.dev) to build a static site to post walk-throughs of how I developed my solutions.

There is a [GitHub action](./.github/workflows/check-build.yml) that runs on a pull request -> main to check everything
is in order. This:

- Builds the project
- Runs the tests
- Builds the docs
- Builds the static site

To enforce these checks the main branch has been protected, and pull requests to main require the action to complete
before they can be merged.

When the pull request is merged into main, a [second GitHub action](./.github/workflows/rust-docs.yml) is triggered.
This:

- Builds the docs
- Builds the static site
- Bundles them into an artefact suitable for GitHub pages
- Deploys the bundle to the repositories [GitHub Pages site](https://kamioftea.github.io/advent-of-code-2024).

## Previous years:

- 2022 `50/50` Rust [GitHub](https://github.com/kamioftea/advent-of-code-2022),
  [Write Ups](https://kamioftea.github.io/advent-of-code-2022/),
  [Puzzles](https://adventofcode.com/2022)
- 2021 `50/50` Rust [GitHub](https://github.com/kamioftea/advent-of-code-2021),
  [Write Ups](https://kamioftea.github.io/advent-of-code-2021/),
  [Puzzles](https://adventofcode.com/2021)
- 2020 `36/50` Rust [GitHub](https://github.com/kamioftea/advent-of-code-2020),
  [Puzzles](https://adventofcode.com/2020)
- 2018 `10/50` Rust [GitHub](https://github.com/kamioftea/advent-of-code-2018),
  [Puzzles](https://adventofcode.com/2018)
- 2017 `50/50` Scala [GitHub](https://github.com/kamioftea/advent-of-code-2017),
  [Write Ups](https://blog.goblinoid.co.uk/tag/advent-of-code-2017/),
  [Puzzles](https://adventofcode.com/2017)
- 2016 `10/50` Scala [GitHub](https://github.com/kamioftea/advent-of-code-2016),
  [Write Ups](https://kamioftea.github.io/advent-of-code-2016/),
  [Puzzles](https://adventofcode.com/2016)
  
