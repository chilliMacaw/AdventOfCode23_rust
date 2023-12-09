#![allow(dead_code)]
#![allow(unused_variables)]
use regex::Regex;
use std::fs::read_to_string;

fn input_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let result_01 = part_one("src/input.txt");
    let result_02 = part_two("src/input.txt");
    println!("result 01: {}", result_01);
    println!("result 02: {}", result_02);
}

fn parser(path: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"-?\d+").unwrap();
    let lines = input_lines(path);
    let mut histories = Vec::new();

    for line in lines {
        let curr_history: Vec<i32> = re
            .find_iter(&line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        histories.push(curr_history);
    }

    histories
}

fn extend_history(hist: Vec<i32>) -> i32 {
    let x = hist.last().unwrap().clone();
    x + extend_history_helper(hist)
}

fn extend_history_helper(hist: Vec<i32>) -> i32 {
    let mut gaps_in_hist: Vec<i32> = Vec::new();
    for i in 0..(hist.len() - 1) {
        gaps_in_hist.push((hist[i + 1] - hist[i]).try_into().unwrap());
    }
    if gaps_in_hist.iter().all(|&val| val == 0) {
        return 0;
    } else if gaps_in_hist.len() == 1 {
        return *gaps_in_hist.first().unwrap();
    }
    let last = gaps_in_hist.last().unwrap().clone();

    last + extend_history_helper(gaps_in_hist)
}

fn part_one(path: &str) -> i32 {
    let histories = parser(path);
    let mut sum = 0;
    for hist in histories {
        sum += extend_history(hist);
    }
    sum
}

fn prepend_history(hist: Vec<i32>) -> i32 {
    let x = hist.first().unwrap().clone();
    x - prepend_history_helper(hist)
}

fn prepend_history_helper(hist: Vec<i32>) -> i32 {
    let mut gaps_in_hist: Vec<i32> = Vec::new();
    for i in 0..(hist.len() - 1) {
        gaps_in_hist.push((hist[i + 1] - hist[i]).try_into().unwrap());
    }
    if gaps_in_hist.iter().all(|&val| val == 0) {
        return 0;
    } else if gaps_in_hist.len() == 1 {
        return *gaps_in_hist.first().unwrap();
    }
    let first = gaps_in_hist.first().unwrap().clone();

    first - prepend_history_helper(gaps_in_hist)
}

fn part_two(path: &str) -> i32 {
    let histories = parser(path);
    let mut sum = 0;
    for hist in histories {
        sum += prepend_history(hist);
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(part_one("src/sample.txt"), 158);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(part_two("src/sample.txt"), -3);
    }
}

// 1882395907
// 1005
