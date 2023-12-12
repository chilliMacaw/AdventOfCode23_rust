#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, fs::read_to_string, result};

fn input_lines(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

type Blocks = Vec<i32>;
type Hotsprings = Vec<char>;

fn main() {
    let result_01 = part_one("src/input.txt");
    let result_02 = part_two("src/input.txt");
    println!("result 01: {}", result_01);
    println!("result 02: {}", result_02);
}

fn parser(path: &str) -> Vec<(Hotsprings, Blocks)> {
    let mut parsed: Vec<(Hotsprings, Blocks)> = Vec::new();
    let lines = input_lines(path);
    for line in input_lines(path) {
        let binding: Vec<&str> = line.split(" ").collect();
        let numbers: Blocks = binding[1]
            .split(',')
            .map(|val| val.parse::<i32>().unwrap())
            .collect();
        let springs: Hotsprings = binding[0].chars().collect();
        parsed.push((springs, numbers));
    }

    parsed
}

fn is_valid(springs: Hotsprings, blocks: Blocks) -> bool {
    let mut count = 0;
    let mut seen: Vec<i32> = vec![];

    for ch in springs {
        if ch == '.' {
            if count > 0 {
                seen.push(count)
            }
            count = 0;
        } else if ch == '#' {
            count += 1;
        } else {
            return false;
        }
    }
    if count > 0 {
        seen.push(count)
    }
    return seen.eq(&blocks);
}

fn num_possibile_solutions(springs: Hotsprings, blocks: Blocks, i: usize) -> i32 {
    if i == springs.len() {
        return if is_valid(springs, blocks) { 1 } else { 0 };
    }
    if springs[i] == '?' {
        let mut tmp_a: Vec<char> = springs[..i].to_vec();
        tmp_a.push('#');
        let mut tmp_b: Vec<char> = springs[..i].to_vec();
        tmp_b.push('.');

        return num_possibile_solutions(
            tmp_a
                .iter()
                .chain(springs[(i + 1)..].iter())
                .map(|val| *val)
                .collect::<Hotsprings>(),
            blocks.clone(),
            i,
        ) + num_possibile_solutions(
            tmp_b
                .iter()
                .chain(springs[(i + 1)..].iter())
                .map(|val| *val)
                .collect::<Hotsprings>(),
            blocks,
            i,
        );
    } else {
        return num_possibile_solutions(springs, blocks, i + 1);
    }
}

fn part_one(path: &str) -> i32 {
    let spring_rows: Vec<(Hotsprings, Blocks)> = parser(path);

    let result = spring_rows
        .iter()
        .map(|(springs, blocks)| num_possibile_solutions(springs.to_vec(), blocks.to_vec(), 0));

    result.sum()
}

fn get_pattern(springs: Hotsprings) -> String {
    if springs.is_empty() {
        return String::from("");
    }
    let mut iterator = springs.iter();
    let mut pattern = String::from("");

    let mut run = true;
    let mut ch = iterator.next().unwrap();
    let mut count = 0;

    while run {
        if *ch == '#' {
            pattern.push('#');
            count = 0;
        } else {
            if count == 0 {
                pattern.push('x');
            }
            count += 1;
        }
        match iterator.next() {
            Some(x) => {
                ch = x;
            }
            None => {
                run = false;
            }
        }
    }
    pattern
}

fn num_possibile_solutions_two(
    springs: Hotsprings,
    blocks: Blocks,
    i_springs: usize,
    i_blocks: usize,
    curr_b_lenght: usize,
    lookup: &mut HashMap<(usize, usize, usize), i64>,
) -> i64 {
    let key = (i_springs, i_blocks, curr_b_lenght);
    match lookup.get(&key) {
        Some(x) => return *x,
        None => {}
    }
    if i_springs == springs.len() {
        if i_blocks == blocks.len() && curr_b_lenght == 0 {
            return 1;
        } else if i_blocks == blocks.len() - 1 && blocks[i_blocks] == (curr_b_lenght as i32) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut ans: i64 = 0;

    for ch in ['.', '#'] {
        if springs[i_springs] == ch || springs[i_springs] == '?' {
            if ch == '.' && curr_b_lenght == 0 {
                ans += num_possibile_solutions_two(
                    springs.clone(),
                    blocks.clone(),
                    i_springs + 1,
                    i_blocks,
                    0,
                    lookup,
                );
            } else if ch == '.'
                && curr_b_lenght > 0
                && i_blocks < blocks.len()
                && blocks[i_blocks] == curr_b_lenght.try_into().unwrap()
            {
                ans += num_possibile_solutions_two(
                    springs.clone(),
                    blocks.clone(),
                    i_springs + 1,
                    i_blocks + 1,
                    0,
                    lookup,
                );
            } else if ch == '#' {
                ans += num_possibile_solutions_two(
                    springs.clone(),
                    blocks.clone(),
                    i_springs + 1,
                    i_blocks,
                    curr_b_lenght + 1,
                    lookup,
                );
            }
        }
    }
    lookup.insert(key, ans);
    ans
}

fn part_two(path: &str) -> i64 {
    let mut spring_rows: Vec<(Hotsprings, Blocks)> = parser(path);

    for (springs, blocks) in spring_rows.iter_mut() {
        let repeated_springs = vec![String::from_iter(springs.iter()); 5];
        let result = repeated_springs.join("?");
        *springs = result.chars().collect();
        let l = blocks.len();
        let repeqated_blocks = blocks.iter().cycle().take(5 * l).collect::<Vec<&i32>>();
        *blocks = repeqated_blocks
            .iter()
            .map(|&val| *val)
            .collect::<Vec<i32>>();
    }

    let result = spring_rows
        .iter()
        .map(|(springs, blocks)| {
            {
                let mut pattern_lookup: HashMap<(usize, usize, usize), i64> = HashMap::new(); // the amount of . is irrelevant to i will change ... to x or . to x or ....... to x for ex: ..##...# = x##x#

                num_possibile_solutions_two(
                    springs.to_vec(),
                    blocks.to_vec(),
                    0,
                    0,
                    0,
                    &mut pattern_lookup,
                )
            }
        })
        .collect::<Vec<i64>>();
    //println!("{:?}", result);

    result.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(part_one("src/sample.txt"), 21);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(part_two("src/sample.txt"), 525152);
    }
}

/* result 01: 7670
result 02: 157383940585037 */
