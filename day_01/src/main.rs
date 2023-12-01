use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn input_lines() -> Result<Vec<String>, std::io::Error> {
    let f = File::open("src/input.txt")?;
    let reader = BufReader::new(f);
    let mut result: Vec<String> = Vec::new();
    for line in reader.lines() {
        result.push(line?)
    }

    return Result::Ok(result);
}

fn get_first(line: &str) -> Option<i32> {
    let mut line_copy = line.to_owned().chars().collect::<Vec<char>>();
    while !line_copy.is_empty() {
        match &line_copy as &[char] {
            ['1', ..] => return Some(1),
            ['o', 'n', 'e', ..] => return Some(1),
            ['2', ..] => return Some(2),
            ['t', 'w', 'o', ..] => return Some(2),
            ['3', ..] => return Some(3),
            ['t', 'h', 'r', 'e', 'e', ..] => return Some(3),
            ['4', ..] => return Some(4),
            ['f', 'o', 'u', 'r', ..] => return Some(4),
            ['5', ..] => return Some(5),
            ['f', 'i', 'v', 'e', ..] => return Some(5),
            ['6', ..] => return Some(6),
            ['s', 'i', 'x', ..] => return Some(6),
            ['7', ..] => return Some(7),
            ['s', 'e', 'v', 'e', 'n', ..] => return Some(7),
            ['8', ..] => return Some(8),
            ['e', 'i', 'g', 'h', 't', ..] => return Some(8),
            ['9', ..] => return Some(9),
            ['n', 'i', 'n', 'e', ..] => return Some(9),
            _ => (),
        }
        line_copy.remove(0);
    }
    None
}

fn get_last(line: &str) -> Option<i32> {
    let mut line_copy = line.to_owned().chars().collect::<Vec<char>>();
    while !line_copy.is_empty() {
        match &line_copy as &[char] {
            [.., '1'] => return Some(1),
            [.., 'o', 'n', 'e'] => return Some(1),
            [.., '2'] => return Some(2),
            [.., 't', 'w', 'o'] => return Some(2),
            [.., '3'] => return Some(3),
            [.., 't', 'h', 'r', 'e', 'e'] => return Some(3),
            [.., '4'] => return Some(4),
            [.., 'f', 'o', 'u', 'r'] => return Some(4),
            [.., '5'] => return Some(5),
            [.., 'f', 'i', 'v', 'e'] => return Some(5),
            [.., '6'] => return Some(6),
            [.., 's', 'i', 'x'] => return Some(6),
            [.., '7'] => return Some(7),
            [.., 's', 'e', 'v', 'e', 'n'] => return Some(7),
            [.., '8'] => return Some(8),
            [.., 'e', 'i', 'g', 'h', 't'] => return Some(8),
            [.., '9'] => return Some(9),
            [.., 'n', 'i', 'n', 'e'] => return Some(9),
            _ => (),
        }
        line_copy.remove(line_copy.len() - 1);
    }
    None
}

fn get_number_form_str(line: &str) -> i32 {
    let first = match get_first(line) {
        Some(val) => val,
        None => -1,
    };
    let last = match get_last(line) {
        Some(val) => val,
        None => -1,
    };
    //println!("last: {}, fist: {}", last, first);
    return (first.to_string() + &last.to_string())
        .parse::<i32>()
        .unwrap();
}

fn main() {
    let mut decoded: Vec<i32> = Vec::new();
    for line in input_lines().unwrap() {
        decoded.push(get_number_form_str(&line))
    }
    println!("{}", decoded.iter().sum::<i32>())
}
