use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashMap;

fn input_lines() -> Result<Vec<String>, std::io::Error> {
    let f = File::open("src/input.txt")?;
    let reader = BufReader::new(f);
    let mut result: Vec<String> = Vec::new();
    result.push("...............................................................................................................................................".to_string());
    for line in reader.lines() {
        result.push(".".to_owned() + &line? + ".")
    }
    result.push("...............................................................................................................................................".to_string());

    return Result::Ok(result);
}

fn check_environment(
    h_map: HashMap<(usize, usize), char>,
    env: ((usize, usize), (usize, usize)),
) -> bool {
    for i in env.0 .0..=env.1 .0 {
        for k in env.0 .1..=env.1 .1 {
            match h_map.get(&(i, k)) {
                Some(_) => return true,
                None => {}
            }
        }
    }
    false
}

fn main() {
    // creatre lookuptabel for symbols by position
    let mut symbol_lookup_table_by_position: HashMap<(usize, usize), char> = HashMap::new();
    for (row_index, line) in input_lines().expect("err: no input").iter().enumerate() {
        line.chars().enumerate().for_each(|(i, c)| match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {}
            _ => {
                symbol_lookup_table_by_position.insert((row_index, i), c);
            }
        });
    }
    // create list of numbers and their environmetns
    let mut numbers_and_environemtns: Vec<(i32, ((usize, usize), (usize, usize)))> = Vec::new();
    for i in 0..input_lines().expect("no input").len() {
        let line = input_lines()
            .expect("no input")
            .get(i)
            .unwrap()
            .chars()
            .collect::<Vec<char>>();
        let mut k = 0;
        while k < line.len() {
            let mut s: String = String::new();
            let start: (i32, i32) = (i as i32, k as i32);
            let has_found = line[k].is_numeric();
            while line[k].is_numeric() {
                s.push(line[k]);
                k = k + 1
            }
            let end: (i32, i32) = (i as i32, (k as i32) - 1);
            if has_found {
                numbers_and_environemtns.push((
                    s.parse::<i32>().unwrap_or(0),
                    (
                        (
                            (start.0 - 1).try_into().unwrap(),
                            (start.1 - 1).try_into().unwrap(),
                        ),
                        (
                            (end.0 + 1).try_into().unwrap(),
                            (end.1 + 1).try_into().unwrap(),
                        ),
                    ),
                ))
            }

            k = k + 1
        }
    }

    let sum = numbers_and_environemtns
        .iter()
        .filter(|env| check_environment(symbol_lookup_table_by_position.clone(), env.1))
        .map(|env| env.0)
        .sum::<i32>();
    println!("{:?}", sum);

    // part two

    let mut numbers_near_gears = numbers_and_environemtns
        .iter()
        .map(|env| {
            (
                env.0,
                check_environment_for_gears(symbol_lookup_table_by_position.clone(), env.1),
            )
        })
        .filter(|res| res.1 .0)
        .map(|(num, is)| (num, is.1))
        .collect::<Vec<_>>();
    let mut sum_two = 0;
    while !numbers_near_gears.is_empty() {
        let curr = numbers_near_gears.pop().unwrap();
        let index = numbers_near_gears
            .iter()
            .position(|elem| curr.1.eq(&elem.1));
        match index {
            Some(x) => {
                let other = numbers_near_gears.remove(x);
                sum_two += curr.0 * other.0;
            }
            None => {}
        }
    }

    println!("part 02* {:?}", sum_two)
}

fn check_environment_for_gears(
    h_map: HashMap<(usize, usize), char>,
    env: ((usize, usize), (usize, usize)),
) -> (bool, (usize, usize)) {
    for i in env.0 .0..=env.1 .0 {
        for k in env.0 .1..=env.1 .1 {
            match h_map.get(&(i, k)) {
                Some('*') => return (true, (i, k)),
                _ => {}
            }
        }
    }
    (false, (0, 0))
}

// i padded the input file with a row of dots (.) as the last and first row, as well as left and right col, this makes handleing edge cases simpler (when scanning the environment)
// 514969 part one
// 78915902 part two
