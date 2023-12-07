#![allow(dead_code)]
#![allow(unused_variables)]
use std::cmp::Ordering;
use std::collections::HashMap;
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
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Card {
    Not,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Combo {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn map_to_combo(cards: [Card; 5]) -> Combo {
    let mut count_map: HashMap<Card, i8> = HashMap::new();
    for card in cards {
        count_map.insert(
            card,
            match count_map.get(&card) {
                Some(x) => x + 1,
                None => 1,
            },
        );
    }
    if count_map.iter().any(|(k, v)| *v == 5) {
        return Combo::FiveOfAKind;
    } else if count_map.iter().any(|(k, v)| *v == 4) {
        return Combo::FourOfAKind;
    } else if count_map.iter().any(|(k, v)| *v == 3) && count_map.iter().any(|(k, v)| *v == 2) {
        return Combo::FullHouse;
    } else if count_map.iter().any(|(k, v)| *v == 3) {
        return Combo::ThreeOfAKind;
    } else if count_map
        .iter()
        .filter(|(k, v)| **v == 2)
        .collect::<Vec<_>>()
        .len()
        == 2
    {
        return Combo::TwoPair;
    } else if count_map.iter().any(|(k, v)| *v == 2) {
        return Combo::OnePair;
    }

    Combo::HighCard
}

#[derive(Debug)]
struct Bet {
    cards: [Card; 5],
    wager: i32,
}

impl Bet {
    fn new(cards: [Card; 5], wager: i32) -> Bet {
        Bet {
            cards: cards,
            wager: wager,
        }
    }
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> Ordering {
        let s = map_to_combo(self.cards);
        let o = map_to_combo(other.cards);
        if s == o {
            for (s_card, o_card) in zip(self.cards, other.cards) {
                if s_card != o_card {
                    return s_card.cmp(&o_card);
                }
            }
            return Ordering::Greater;
        }
        s.cmp(&o)
    }
}

impl PartialOrd for Bet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bet {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}
impl Eq for Bet {}

fn main() {
    part_one();
    part_two();
}

fn parser() -> Vec<Bet> {
    let mut bets: Vec<Bet> = Vec::new();
    for line in input_lines().expect("noinput") {
        let [first, second, ..] = line.split(" ").collect::<Vec<&str>>()[..] else {
            panic!("unexpected input")
        };
        //println!("{first}, {second}");
        let cards = &first
            .chars()
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => Card::Not,
            })
            .collect::<Vec<Card>>();
        bets.push(Bet::new(
            [cards[0], cards[1], cards[2], cards[3], cards[4]],
            second.parse::<i32>().expect("expected bett amount"),
        ));
    }
    bets
}

fn part_one() {
    let mut bets = parser();
    bets.sort();
    let not = bets
        .iter()
        .filter(|bet| bet.cards.contains(&Card::Not))
        .collect::<Vec<_>>()
        .len();
    //println!("not: {}", not);
    println!("{:?}", bets);
    let sum: i64 = bets
        .iter()
        .enumerate()
        .map(|(i, bet)| (i as i64 + 1) * bet.wager as i64)
        .sum();

    println!("solution part 01: {} mit länge {}", sum, bets.len())
}
fn part_two() {}
