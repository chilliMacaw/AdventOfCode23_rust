use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;

macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

/*
    compose!(multiply, add, divide, subtract);
    means multiply is fist, add second, divide third and subtract last
*/
fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

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

fn exec_correct_mapping(mapping: &Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)>, num: i64) -> i64 {
    for (map, start, len) in mapping {
        if *start <= num && num < start + len {
            return map(num);
        }
    }
    num
}

fn part_one() {
    let re = Regex::new(r"\d+").unwrap();
    let binding = input_lines().expect("no input");
    let mut input_iterator = binding.iter();
    let seed_line = input_iterator
        .next()
        .expect("no seeds")
        .split(":")
        .last()
        .unwrap()
        .trim();

    let seeds: Vec<i64> = re
        .find_iter(seed_line)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    input_iterator.next();
    input_iterator.next();

    // 1. mapping

    let create_mapping = |dst: i64, start: i64| move |num: i64| dst + (num - start);
    let mut line = input_iterator.next();
    let mut seed_to_soil_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        seed_to_soil_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 2. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut soil_to_fertalizer_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        soil_to_fertalizer_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 3. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut fertalizer_to_water_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        fertalizer_to_water_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 4. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut water_to_light_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        water_to_light_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 5. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut light_to_tempreture_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        light_to_tempreture_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 6. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut temptreture_to_humidity_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        temptreture_to_humidity_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }

    // 7. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut humidity_to_location_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap_or(&String::new()).is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        humidity_to_location_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }

    let first = |num: i64| exec_correct_mapping(&seed_to_soil_mappings, num);
    let second = |num: i64| exec_correct_mapping(&soil_to_fertalizer_mappings, num);
    let third = |num: i64| exec_correct_mapping(&fertalizer_to_water_mappings, num);
    let fourth = |num: i64| exec_correct_mapping(&water_to_light_mappings, num);
    let fith = |num: i64| exec_correct_mapping(&light_to_tempreture_mappings, num);
    let sixth = |num: i64| exec_correct_mapping(&temptreture_to_humidity_mappings, num);
    let seventh = |num: i64| exec_correct_mapping(&humidity_to_location_mappings, num);
    let ultiamte_mapping = compose!(first, second, third, fourth, fith, sixth, seventh);
    let mut min_location: i64 = std::i64::MAX;
    for seed in seeds {
        let curr = ultiamte_mapping(seed);
        min_location = cmp::min(min_location, curr);
    }
    println!("minimum loation part 01: {}", min_location)
}

fn part_two() {
    let re = Regex::new(r"\d+").unwrap();
    let binding = input_lines().expect("no input");
    let mut input_iterator = binding.iter();
    let seed_line = input_iterator
        .next()
        .expect("no seeds")
        .split(":")
        .last()
        .unwrap()
        .trim();

    let mut seeds_tmp: Vec<i64> = re
        .find_iter(seed_line)
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    let mut seeds: Vec<Range<i64>> = Vec::new();

    while seeds_tmp.len() > 0 {
        let len = seeds_tmp.pop().unwrap();
        let start = seeds_tmp.pop().unwrap();
        seeds.push(start..start + len - 1);
    }

    input_iterator.next();
    input_iterator.next();

    // 1. mapping

    let create_mapping = |dst: i64, start: i64| move |num: i64| dst + (num - start);
    let mut line = input_iterator.next();
    let mut seed_to_soil_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        seed_to_soil_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 2. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut soil_to_fertalizer_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        soil_to_fertalizer_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 3. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut fertalizer_to_water_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        fertalizer_to_water_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 4. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut water_to_light_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        water_to_light_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 5. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut light_to_tempreture_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        light_to_tempreture_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }
    // 6. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut temptreture_to_humidity_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap().is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        temptreture_to_humidity_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }

    // 7. mapping
    input_iterator.next();
    line = input_iterator.next();

    let mut humidity_to_location_mappings: Vec<(Box<dyn Fn(i64) -> i64>, i64, i64)> = Vec::new();
    while !line.unwrap_or(&String::new()).is_empty() {
        let current_mapping: Vec<i64> = re
            .find_iter(line.unwrap())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        let dst = current_mapping[0];
        let start = current_mapping[1];
        let len = current_mapping[2];
        humidity_to_location_mappings.push((Box::new(create_mapping(dst, start)), start, len));
        line = input_iterator.next()
    }

    let first = |num: i64| exec_correct_mapping(&seed_to_soil_mappings, num);
    let second = |num: i64| exec_correct_mapping(&soil_to_fertalizer_mappings, num);
    let third = |num: i64| exec_correct_mapping(&fertalizer_to_water_mappings, num);
    let fourth = |num: i64| exec_correct_mapping(&water_to_light_mappings, num);
    let fith = |num: i64| exec_correct_mapping(&light_to_tempreture_mappings, num);
    let sixth = |num: i64| exec_correct_mapping(&temptreture_to_humidity_mappings, num);
    let seventh = |num: i64| exec_correct_mapping(&humidity_to_location_mappings, num);
    let ultiamte_mapping = compose!(first, second, third, fourth, fith, sixth, seventh);
    let mut min_location: i64 = std::i64::MAX;
    let mut i = 1;
    for seed_range in seeds {
        println!("next {i}");
        for seed in seed_range {
            let curr = ultiamte_mapping(seed);
            min_location = cmp::min(min_location, curr);
        }
        i += 1
    }
    println!("minimum loation part 02: {}", min_location)
}
//solution to part 02  15880236 took 40 minutes to compute without rust optimizations
//With release optimizations, it ran in 2 min 13 seconds.
// solution to part 01: 621354867
