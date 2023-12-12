#![allow(dead_code)]
#![allow(unused_variables)]
use grid::Grid;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

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
#[derive(PartialEq, Clone, Copy)]
enum Sector {
    Galaxy(i32),
    Void,
}

type Position = (usize, usize);
type Galaxy = (i32, Position);

struct ObservatoryMap {
    galaxies: HashSet<Galaxy>,
    grid: Grid<Sector>,
}

fn parser(path: &str) -> ObservatoryMap {
    let lines = input_lines(path);
    let galaxy_set: HashSet<Galaxy> = HashSet::new();
    let grid_n: Vec<Vec<Sector>> = Vec::new();

    let mut intermediate_grid: Vec<Vec<Sector>> = Vec::new();

    let mut count = 0;
    for (y, line) in input_lines(path).iter().enumerate() {
        intermediate_grid.push(Vec::new());
        for (x, elem) in line.chars().enumerate() {
            let next: Sector = match elem {
                '#' => {
                    count += 1;
                    Sector::Galaxy(count)
                }
                _ => Sector::Void, // '.'
            };
            intermediate_grid[y].push(next);
        }
    }
    let cols = intermediate_grid[0].len();
    let row_l = intermediate_grid[0].len();
    let col_l = intermediate_grid.len();

    let binding = intermediate_grid
        .into_iter()
        .flatten()
        .collect::<Vec<Sector>>();
    let observatory_map: ObservatoryMap = ObservatoryMap {
        galaxies: galaxy_set,
        grid: Grid::from_vec(binding, cols),
    };

    observatory_map
}

fn part_one(path: &str) -> i32 {
    let mut observatory_map = parser(path);
    let row_l = observatory_map.grid.rows();
    let col_l = observatory_map.grid.cols();
    let mut new_cols: Vec<usize> = Vec::new();
    let mut new_rows: Vec<usize> = Vec::new();

    for (i, mut row) in observatory_map.grid.iter_rows().enumerate() {
        if row.all(|s| *s == Sector::Void) {
            new_rows.push(i);
        }
    }
    for (i, mut col) in observatory_map.grid.iter_cols().enumerate() {
        if col.all(|s| *s == Sector::Void) {
            new_cols.push(i);
        }
    }

    new_rows.iter().enumerate().for_each(|(i, index)| {
        observatory_map
            .grid
            .insert_row(*index + i, vec![Sector::Void; row_l])
    });
    new_cols.iter().enumerate().for_each(|(i, index)| {
        observatory_map
            .grid
            .insert_col(*index + i, vec![Sector::Void; col_l + new_rows.len()])
    });

    for (y, row) in observatory_map.grid.iter_rows().enumerate() {
        for (x, item) in row.enumerate() {
            match item {
                Sector::Galaxy(val) => {
                    observatory_map.galaxies.insert((*val, (y, x)));
                }
                Sector::Void => {}
            }
        }
    }

    for (y, row) in observatory_map.grid.iter_rows().enumerate() {
        for (x, item) in row.enumerate() {
            print!(
                "{}",
                match item {
                    Sector::Galaxy(x) => x.to_string(),
                    Sector::Void => ".".to_string(),
                }
            )
        }
        print!("\n")
    }
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new(); // key is always sortet inacendig order key: (1,2) existes but key (2,1) cant exsist

    for (galaxy, (y, x)) in observatory_map.galaxies.iter() {
        let x = breath_first_search((*y, *x), *galaxy, &observatory_map);
        for (i, dist) in x.iter().enumerate() {
            let key = (
                (*galaxy).min((i + 1).try_into().unwrap()),
                (*galaxy).max((i + 1).try_into().unwrap()),
            );
            distances.insert(key, *dist);
        }
    }
    //println!("distances: {:?}", distances.iter().collect::<Vec<_>>());
    distances.values().sum()
}

fn breath_first_search(start: Position, galaxy_id: i32, map: &ObservatoryMap) -> Vec<i32> {
    let mut curr_dist = 0;
    let mut dists: Vec<i32> = (0..map.galaxies.len()).map(|y| i32::MAX).collect();
    dists[(galaxy_id - 1) as usize] = 0;
    let mut explored: HashSet<(isize, isize)> = HashSet::new();
    let mut newly_discoverd: Vec<(isize, isize)> = Vec::new();
    newly_discoverd.push((start.0 as isize, start.1 as isize));
    explored.insert((start.0 as isize, start.1 as isize));
    //println!("start:{:?}", start.clone());

    let mut new_newly_discoverd: Vec<Vec<(isize, isize)>> = Vec::new();
    new_newly_discoverd.push(newly_discoverd.clone());
    while !new_newly_discoverd.is_empty() {
        curr_dist += 1;
        newly_discoverd = new_newly_discoverd.remove(0);
        let mut tmp: Vec<(isize, isize)> = Vec::new();
        while !newly_discoverd.is_empty() {
            let next = newly_discoverd.remove(0);
            //println!("next:{:?}", next.clone());

            for (adj_y, adj_x) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if next.0 + adj_y >= 0
                    && next.1 + adj_x >= 0
                    && next.0 + adj_y < map.grid.rows().try_into().unwrap()
                    && next.1 + adj_x < map.grid.cols().try_into().unwrap()
                {
                    if !explored.contains(&(next.0 + adj_y, next.1 + adj_x)) {
                        let new_node = (next.0 + adj_y, next.1 + adj_x);
                        explored.insert(new_node);
                        tmp.push(new_node);
                        //println!("new_node:{:?}", new_node.clone());
                        let new_sector: &Sector = map
                            .grid
                            .get(new_node.0 as usize, new_node.1 as usize)
                            .unwrap();
                        match new_sector {
                            Sector::Galaxy(x) => {
                                dists[(*x - 1) as usize] = curr_dist;
                            }
                            Sector::Void => {}
                        }
                    }
                }
            }
        }
        if !tmp.is_empty() {
            new_newly_discoverd.push(tmp);
        }
    }

    dists
}

// useing Manhatten distance for part 02 completey diffrent apporoach
fn part_two(path: &str) -> i128 {
    const EXPANSION_RATE: usize = 999999;
    let mut observatory_map = parser(path);
    let mut galaxy_hashmap: HashMap<i32, (usize, usize)> = HashMap::new();

    for (y, row) in observatory_map.grid.iter_rows().enumerate() {
        for (x, item) in row.enumerate() {
            match item {
                Sector::Galaxy(val) => {
                    observatory_map.galaxies.insert((*val, (y, x)));
                }
                Sector::Void => {}
            }
        }
    }

    observatory_map.galaxies.iter().for_each(|(id, (y, x))| {
        galaxy_hashmap.insert(*id, (*y, *x));
    });

    // looking for empty rows and cols
    let mut new_cols: Vec<(usize, usize)> = Vec::new();
    let mut new_rows: Vec<(usize, usize)> = Vec::new();

    let mut count = 0;
    for (i, mut row) in observatory_map.grid.iter_rows().enumerate() {
        if row.all(|s| *s == Sector::Void) {
            new_rows.push((i, count));
            count += 1;
        }
    }
    count = 0;
    for (i, mut col) in observatory_map.grid.iter_cols().enumerate() {
        if col.all(|s| *s == Sector::Void) {
            new_cols.push((i, count));
            count += 1;
        }
    }

    // expanding the rows
    galaxy_hashmap.iter_mut().for_each(|(galaxy_id, (y, x))| {
        let row_expansion: usize = new_rows
            .iter()
            .map(|(row_i, _)| {
                if *row_i < *y {
                    return EXPANSION_RATE;
                } else {
                    return 0;
                }
            })
            .sum();

        *y = y.clone() + row_expansion;
    });
    // expanding the cols
    galaxy_hashmap.iter_mut().for_each(|(galaxy_id, (y, x))| {
        let col_expansion: usize = new_cols
            .iter()
            .map(|(col_i, _)| {
                if *col_i < *x {
                    return EXPANSION_RATE;
                } else {
                    return 0;
                }
            })
            .sum();
        *x = x.clone() + col_expansion;
    });

    let mut galaxy_pairs: HashSet<(i32, i32)> = HashSet::new();
    for (fst, snd) in galaxy_hashmap
        .keys()
        .cartesian_product(galaxy_hashmap.keys())
    {
        galaxy_pairs.insert((*fst.min(snd), *fst.max(snd)));
    }

    let res = galaxy_pairs
        .iter()
        .map(|(fst, snd)| {
            (
                galaxy_hashmap.get(fst).unwrap(),
                galaxy_hashmap.get(snd).unwrap(),
            )
        })
        .map(|((fst_y, fst_x), (snd_y, snd_x))| {
            (fst_y.abs_diff(*snd_y) + fst_x.abs_diff(*snd_x)) as i128
        })
        .collect::<Vec<_>>();
    println!("hashset: {:?}", res);
    res.iter().sum::<i128>()

    //println!("galaxy hasmap: {:?}", galaxy_hashmap);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_a() {
        assert_eq!(part_one("src/sample.txt"), 374);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(part_two("src/sample.txt"), 8410);
    }
}

// 9370588
// 746207878188
/*
....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
*/
