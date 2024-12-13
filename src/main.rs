extern crate cached;
extern crate core;
extern crate itertools;
#[macro_use]
extern crate text_io;
mod bootstrap_day;
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod helpers;

use bootstrap_day::bootstrap_day;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    print!("Which day? (0 to run all): ");
    io::stdout().flush().unwrap();

    let day: u8 = read!();
    let days: Vec<Box<dyn Fn() -> ()>> = vec![
        Box::new(|| day_1::run()),
        Box::new(|| day_2::run()),
        Box::new(|| day_3::run()),
        Box::new(|| day_4::run()),
        Box::new(|| day_5::run()),
        Box::new(|| day_6::run()),
        Box::new(|| day_7::run()),
        Box::new(|| day_8::run()),
        Box::new(|| day_9::run()),
        Box::new(|| day_10::run()),
        Box::new(|| day_11::run()),
        Box::new(|| day_12::run()),
    ];

    let start = Instant::now();
    match days.get((day - 1) as usize) {
        Some(solution) => solution(),
        None if day == 0 => days.iter().enumerate().for_each(|(i, solution)| {
            let start = Instant::now();
            println!("==== Day {} ====", i + 1);
            solution();
            println!("-- took {:.2?}", start.elapsed());
        }),
        None if day >= 1 && day <= 25 => bootstrap_day(day).expect("Failed to bootstrap day"),
        None => println!("Invalid Day {}", day),
    }

    println!();
    println!("Finished in {:.2?}", start.elapsed());
}
