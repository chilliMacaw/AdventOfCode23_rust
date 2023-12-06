use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::zip;

fn input_lines() -> Result<Vec<String>, std::io::Error> {
    let f = File::open("src/input.txt")?;
    let reader = BufReader::new(f);
    let mut result: Vec<String> = Vec::new();
    for line in reader.lines() {
        result.push(line?)
    }

    return Result::Ok(result);
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let re = Regex::new(r"\d+").unwrap();

    let durations = re
        .find_iter(input_lines().expect("no input").get(0).unwrap())
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let distances = re
        .find_iter(input_lines().expect("no input").get(1).unwrap())
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let hold_upper_bound = zip(durations.clone(), distances.clone())
        .map(|(t, d)| {
            ((((t as f64) * (t as f64) - (4.0 * d as f64)).sqrt() + t as f64) / 2.0) as i32
        })
        .collect::<Vec<_>>();
    let hold_lower_bound = zip(durations, distances)
        .map(|(t, d)| {
            let mut lower = (-((t as f64) * (t as f64) - (4.0 * d as f64)).sqrt() + t as f64) / 2.0;
            if lower == (lower as u32) as f64 {
                lower += 1.0;
            }
            lower as i32
        })
        .collect::<Vec<_>>();
    let result: i32 = zip(hold_lower_bound, hold_upper_bound)
        .map(|(lo, hi)| hi - lo)
        .product();

    println!("{:?}", result)
}

fn part_two() {
    let re = Regex::new(r"\d+").unwrap();

    let duration = re
        .find_iter(input_lines().expect("no input").get(0).unwrap())
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = re
        .find_iter(input_lines().expect("no input").get(1).unwrap())
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let hold_upper_bound = ((((duration as f64) * (duration as f64) - (4.0 * distance as f64))
        .sqrt()
        + duration as f64)
        / 2.0) as i64;

    let hold_lower_bound = {
        let mut lower = (-((duration as f64) * (duration as f64) - (4.0 * distance as f64)).sqrt()
            + duration as f64)
            / 2.0;
        if lower == (lower as u32) as f64 {
            lower += 1.0;
        }
        lower as i64
    };

    let result: i64 = hold_upper_bound - hold_lower_bound;

    println!("{:?}", result)
}

// use can use fancy math go figure out that result = (amount of natnumber in ) intervall (\fraq{- \sqrt(t^2-4d)+t}{2}),\fraq{\sqrt(t^2-4d)+t}{2}) when t is the given time and d is given the distance
// lower and upper bound are exclusive
// solution part 01 3316275
// solution part 02 27102791
