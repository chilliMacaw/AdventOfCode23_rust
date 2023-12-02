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
#[derive(Debug)]
struct Game<'a> {
    id: u32,
    draws: Vec<Vec<(i32, &'a str)>>,
}

fn is_draw_possible(draw: &(i32, &str)) -> bool {
    match draw {
        &(x, "blue") => return x < 15,
        &(x, "red") => return x < 13,
        &(x, "green") => return x < 14,
        _ => {}
    }
    false
}

fn main() {
    let mut _all_games: Vec<Game> = Vec::new();
    for line in input_lines().unwrap() {
        let mut game = Game {
            id: 0,
            draws: Vec::new(),
        };
        let tmp = line.split(':').collect::<Vec<&str>>();
        game.id = tmp
            .clone()
            .first()
            .expect("parse error no first element")
            .trim_matches(char::is_alphabetic)
            .trim()
            .parse::<u32>()
            .expect("i did go wrong here");

        tmp.last()
            .expect("no draws")
            .split(';')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|val| {
                let draw = val.split(',').collect::<Vec<&str>>();
                game.draws.push(
                    draw.iter()
                        .map(|val| {
                            (
                                val.trim_matches(char::is_alphabetic)
                                    .trim()
                                    .parse::<i32>()
                                    .expect("no amount given"),
                                if val.ends_with("blue") {
                                    "blue"
                                } else if val.ends_with("red") {
                                    "red"
                                } else {
                                    "green"
                                },
                            )
                        })
                        .collect::<Vec<(i32, &str)>>(),
                )
            });
        _all_games.push(game);
    }
    //println!("{:?}", _all_games);
    let possible_games = _all_games.iter().filter(|game: &&Game<'_>| {
        game.draws
            .iter()
            .all(|item| item.iter().all(is_draw_possible))
    });
    println!(
        "sum of game Ids: {}",
        possible_games.map(|g| g.id).sum::<u32>()
    );
    let game_powers = _all_games.iter().map(|game| {
        min_power(&game.draws, "blue")
            * min_power(&game.draws, "red")
            * min_power(&game.draws, "green")
    });

    println!("sum of game powers: {}", game_powers.sum::<i32>());
}

fn min_power(draws: &Vec<Vec<(i32, &str)>>, color: &str) -> i32 {
    return draws.iter().fold(0, |acc, val| {
        std::cmp::max(
            acc,
            val.iter()
                .filter(|(_x, s)| s.contains(color))
                .collect::<Vec<_>>()
                .first()
                .unwrap_or(&&(0, color))
                .0,
        )
    });
}
// solution part 01 3059
// solution part 02 65371
