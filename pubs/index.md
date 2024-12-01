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

I'm sticking with Rust as I enjoy using it for these types of puzzles, and it's not something I get to use day-to-day.
I also think I still have a lot to learn about using Rust. I'm starting from a skeleton version of the repository I
set up last year, and will look at iterating on it as the challenge continues. 

This has the following features:
- [A documentation site](./advent_of_code_2024/) built using the `cargo doc` tool bundled with Rust.  
- This static site, built with [11ty](https://www.11ty.dev) where I can write up how I've tackled each puzzle.
- A GitHub Actions workflow test PRs compile and the static site builds .
- A second workflow to publish both documentation and write-ups to GitHub Pages when a PR is merged into main.

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
