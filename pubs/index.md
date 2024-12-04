---
header: Jeff's Advent of Code 2024
title: 'Solutions List | Advent of Code 2024 | Jeff Horton'
description: |
  I'm attempting Advent of Code 2024 in Rust. This page links to my solutions and write-ups for each day's 
  puzzle.
---

[Advent of Code](https://adventofcode.com/2024) is a yearly challenge with one coding puzzle a day from 1st of December
until Christmas Day. The challenges are language agnostic, providing the input as a text file, and expecting a number or
a string as the result of each part.

I have done the last few years in [Rust](https://rustlang.org), and will be continuing to use it this year. This is
mostly a convenience. I have a setup that allows me to solve the puzzles in a TDD style, and publish the code and a
write-up quickly. Whilst I'm more comfortable using Rust for these puzzles, I still feel I have a lot of learning to
do to write Rust well and idiomatically. I considered using this year to learn [Gleam](https://gleam.run/), but I
haven't had the time to set up tooling, so maybe next year?

I have copied over the repo tooling from last year. This has the following features:

- [A documentation site](./advent_of_code_2024/) built using the `cargo doc` tool bundled with Rust.
- This static site, built with [11ty](https://www.11ty.dev) where I can write up how I've tackled each puzzle.
- A GitHub Actions workflow test PRs compile and the static site builds.
- A second workflow to publish both documentation and write-ups to GitHub Pages when a PR is merged into main.

I have developed a pattern for solving the daily puzzles, which I will likely follow again this year. Usually
the puzzle statement is broken into stages with examples. Those examples make for good tests for doing Test Driven
Development. Idiomatically, Rust tests are in a test mod in the same file as the code its testing, so I can iterate
through the steps within one file for the day. Once the tests are passing, running the same against the puzzle input is
hopefully trivial. In cases where the puzzle as described hits performance issues, I have a test harness ready,
which allows refactoring to a more efficient implementation.

I will usually step through the stages of solving the puzzle in the write-up article, posting the implementing
function and associated test.

## My Solutions

<div class="solutions-list">
{% for solution in solutions %}
  <section class="solution" aria-labelledby="{{ solution.title | slugify }}">
    <h3 class="solution-title" id="{{ solution.title | slugify }}">{{solution.title}}</h3>
    <div class="solution-links">
      {%- for label, href in solution.links -%}
        <!--suppress HtmlUnknownTarget -->
        <a href="{{ href | url }}" class="solution-link">{{ label }}</a>
      {%- endfor -%}
    </div>
  </section>
{% endfor %}
</div>
