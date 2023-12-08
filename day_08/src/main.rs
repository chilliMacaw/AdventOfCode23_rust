#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
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

fn parser(path: &str) -> (String, HashMap<String, (String, String)>) {
    let mut desert_map: HashMap<String, (String, String)> = HashMap::new();
    let mut lines = input_lines(path);
    let first = lines.remove(0);
    lines.remove(0);
    for line in lines {
        let mut binding: Vec<&str> = line.split(" = ").collect();
        let key = binding.remove(0).to_owned();
        let tmp = binding.remove(0);
        let mut left = tmp.split(", ").collect::<Vec<_>>()[0].chars();
        let mut right = tmp.split(", ").collect::<Vec<_>>()[1].chars();
        left.next();
        right.next_back();
        let value = (left.as_str().to_owned(), right.as_str().to_owned());
        desert_map.insert(key, value.to_owned());
    }
    (first, desert_map)
}

fn part_two(path: &str) -> i64 {
    let (fst, desert_map): (String, HashMap<String, (String, String)>) = parser(path);

    let current_positions = desert_map
        .iter()
        .filter(|(k, v)| k.ends_with('A'))
        .collect::<Vec<(&String, &(String, String))>>();

    println!("current_positions: {:?}", current_positions);
    let mut counts: HashMap<String, i64> = HashMap::new();
    for (start_src, (start_dst_left, start_dst_right)) in current_positions {
        counts.insert(start_src.to_string(), 0);

        let (mut current_src, (mut dst_left, mut dst_right)) =
            (start_src, (start_dst_left, start_dst_right));

        for next in fst.chars().cycle() {
            if current_src.ends_with('Z') {
                break;
            }
            match next {
                'R' => {
                    current_src = dst_right;
                    let tmp = desert_map.get(dst_right).expect("location not found");
                    dst_left = &tmp.0;
                    dst_right = &tmp.1;
                }
                _ => {
                    current_src = dst_left;
                    let tmp = desert_map.get(dst_left).expect("location not found");
                    dst_left = &tmp.0;
                    dst_right = &tmp.1;
                }
            }

            counts.entry(start_src.to_string()).and_modify(|e| *e += 1);
        }
    }

    counts
        .values()
        .map(|&a| a)
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap()
}

fn part_one(path: &str) -> i32 {
    let (fst, desert_map): (String, HashMap<String, (String, String)>) = parser(path);
    let directions = fst.chars().cycle();
    let mut current_position = String::from("AAA");
    let mut decision = desert_map
        .get(&current_position)
        .expect("locationd oes not exsist");
    let mut count = 0;
    for next in directions {
        if current_position.eq(&String::from("ZZZ")) {
            println!("count : {}", count);
            return count;
        }
        match next {
            'R' => current_position = decision.clone().1,
            _ => current_position = decision.clone().0,
        }
        decision = desert_map
            .get(&current_position)
            .expect("locationd oes not exsist");
        count += 1;
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_solve_b() {
        assert_eq!(part_two("src/sample_two.txt"), 6);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(part_one("src/sample.txt"), 6);
    }
}
