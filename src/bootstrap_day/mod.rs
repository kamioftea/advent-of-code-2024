use error_chain::error_chain;
use itertools::Itertools;
use regex::Regex;
use reqwest::cookie::Jar;
use reqwest::Url;
use scraper::{Html, Selector};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::sync::Arc;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

pub fn bootstrap_day(day: u8) -> Result<()> {
    let session_cookie =
        fs::read_to_string("res/session_cookie.txt").expect("Failed to read session cookie");

    let cookie = format!("session={}; Domain=adventofcode.com", session_cookie);
    let url = "https://www.adventofcode.com".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()?;

    let input_target = format!("https://www.adventofcode.com/2024/day/{}/input", day);
    let input_file_contents = client.get(input_target).send()?.text()?;

    let output_filename = format!("res/day-{}-input.txt", day);
    let mut output_file = File::create(output_filename.clone())?;
    copy(&mut input_file_contents.as_bytes(), &mut output_file)?;

    println!("Puzzle input saved to {}", output_filename);

    let puzzle_target = format!("https://www.adventofcode.com/2024/day/{}", day);
    let input_file_contents = client.get(puzzle_target).send()?.text()?;

    let html = Html::parse_document(input_file_contents.as_str());
    let selector = Selector::parse("article.day-desc > h2").unwrap();
    let pattern = Regex::new(r"^--- Day \d+: (?<title>.*) ---$").unwrap();
    let title = html
        .select(&selector)
        .next()
        .map(|h2| h2.text().join(""))
        .and_then(|text| {
            pattern
                .captures_iter(text.as_str())
                .exactly_one()
                .ok()
                .and_then(|caps| caps.name("title").map(|m| m.as_str().to_string()))
        })
        .unwrap_or("???".to_string())
        .to_string();

    println!("Title: {title}");

    let rust_filename = format!("src/day_{}.rs", day);
    let rust_contents = format!("\
//! This is my solution for [Advent of Code - Day {day}: _{title}_](https://adventofcode.com/2024/day/{day})
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-{day}-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day {day}.
pub fn run() {{
    let _contents = fs::read_to_string(\"res/day-{day}-input.txt\").expect(\"Failed to read file\");
}}

#[cfg(test)]
mod tests {{

}}",
        day=day
    );

    let mut rust_file = File::create(rust_filename.clone())?;
    copy(&mut rust_contents.as_bytes(), &mut rust_file)?;

    println!("Rust file written {}", rust_filename);

    let markdown_filename = format!("pubs/blog/day_{}.md", day);
    let markdown_contents = format!(
        "\
---
day: {day}
tags: [post]
header: 'Day {day}: {title}'
---
",
        day = day
    );

    let mut markdown_file = File::create(markdown_filename.clone())?;
    copy(&mut markdown_contents.as_bytes(), &mut markdown_file)?;

    println!("Blog file written {}", markdown_filename);

    // TODO - modify main.rs

    Ok(())
}
