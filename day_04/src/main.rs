use regex::Regex;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;

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
    part_two()
}

fn winnings_in_card(card: &Card) -> u32 {
    let mut intersection = card
        .winning_numbers
        .intersection(&card.my_numbers)
        .collect::<Vec<_>>();
    let mut result = 1;
    match intersection.pop() {
        Some(_) => intersection.iter().for_each(|_| {
            result = result * 2;
        }),
        None => return 0,
    }
    return result;
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

fn part_one() {
    let re = Regex::new(r"\d+").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for (i, line) in input_lines().expect("no input").iter().enumerate() {
        let tmp = line
            .split(":")
            .last()
            .expect("no numbers")
            .split("|")
            .collect::<Vec<&str>>();
        let winners_tmp = tmp.first().expect("no winning numbers in input");
        let my_numbers_tmp = tmp.last().expect("no my numbers in input");
        let winning_numbers: HashSet<u32> = re
            .find_iter(&winners_tmp)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        let my_numbers: HashSet<u32> = re
            .find_iter(&my_numbers_tmp)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        cards.push(Card {
            id: i,
            winning_numbers,
            my_numbers,
        });
    }
    //println!("Cards: {:?}", cards);
    let sum = cards
        .iter()
        .map(|card| winnings_in_card(&card))
        .sum::<u32>();
    println!("part 01 solution: {:?}", sum);
}

fn intersections_in_card(card: &Card) -> usize {
    let intersection = card
        .winning_numbers
        .intersection(&card.my_numbers)
        .collect::<Vec<_>>();
    return intersection.len();
}

fn walk_the_scratch_cards(index: usize, cards: Rc<RefCell<Vec<(&Card, usize)>>>) -> usize {
    if index == cards.borrow().len() - 1 {
        return cards.borrow()[index].1;
    } else {
        let curr_amt = cards.borrow()[index].1;
        let intersections = intersections_in_card(&cards.borrow()[index].0);
        for k in index + 1..(index + intersections + 1) {
            let mut cards_borrowed = cards.borrow_mut();
            let card = &mut cards_borrowed[k];
            card.1 += 1 * curr_amt;
        }
        //println!("{:?}", cards);
        return walk_the_scratch_cards(index + 1, cards) + curr_amt;
    }
}

fn part_two() {
    let re = Regex::new(r"\d+").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    for (i, line) in input_lines().expect("no input").iter().enumerate() {
        let tmp = line
            .split(":")
            .last()
            .expect("no numbers")
            .split("|")
            .collect::<Vec<&str>>();
        let winners_tmp = tmp.first().expect("no winning numbers in input");
        let my_numbers_tmp = tmp.last().expect("no my numbers in input");

        let winning_numbers: HashSet<u32> = re
            .find_iter(&winners_tmp)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        let my_numbers: HashSet<u32> = re
            .find_iter(&my_numbers_tmp)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        cards.push(Card {
            id: i,
            winning_numbers,
            my_numbers,
        });
    }
    let x = Rc::new(RefCell::new(
        cards
            .iter()
            .map(|val| (val, 1 as usize))
            .to_owned()
            .collect::<Vec<(&Card, usize)>>(),
    ));

    let result = walk_the_scratch_cards(0, x.clone());
    //println!("{:?}", x);
    println!("solution part 02: {:?}", result);
}
