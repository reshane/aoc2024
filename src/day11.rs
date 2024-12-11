use std::collections::HashMap;

pub fn solve() {
    let contents = std::fs::read_to_string("input_11.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let mut stones = HashMap::<i64, i64>::new();
    contents
        .split(" ")
        .map(|s| {
            s.trim().parse::<i64>().expect("Valid ints in the line")
        })
        .for_each(|s| {
            if stones.contains_key(&s) {
                let s_c = stones.get(&s).expect("where'd it go?");
                stones.insert(s, *s_c);
            } else {
                stones.insert(s, 1);
            }
        });
    solve_n(75, stones)
}

fn solve_n(n: usize, mut stones: HashMap<i64, i64>) -> i64 {
    for _ in 0..n {
        let mut n_stones = HashMap::<i64, i64>::new();
        for stone in stones.keys() {
            let coef = stones.get(stone).expect("where did stone go?");
            if *stone == 0 {
                if n_stones.contains_key(&1) {
                    let o_c = n_stones.get(&1).expect("where did left go?");
                    n_stones.insert(1, o_c + coef);
                } else {
                    n_stones.insert(1, *coef);
                }
            } else if format!("{stone}").len() % 2 == 0 {
                let stone_str = format!("{stone}");
                let left = stone_str[0..stone_str.len()/2]
                    .parse::<i64>().unwrap();
                let right = stone_str[stone_str.len()/2..]
                    .parse::<i64>().unwrap();
                if n_stones.contains_key(&left) {
                    let l_c = n_stones.get(&left).expect("where did left go?");
                    n_stones.insert(left, l_c + coef);
                } else {
                    n_stones.insert(left, *coef);
                }
                if n_stones.contains_key(&right) {
                    let r_c = n_stones.get(&right).expect("where did right go?");
                    n_stones.insert(right, r_c + coef);
                } else {
                    n_stones.insert(right, *coef);
                }
            } else {
                let new = stone * 2024;
                if n_stones.contains_key(&new) {
                    let n_c = n_stones.get(&new).expect("where did new go?");
                    n_stones.insert(new, n_c + coef);
                } else {
                    n_stones.insert(new, *coef);
                }
            }
        }
        stones = n_stones.clone();
    }
    stones
        .values()
        .sum()
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_11.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 65601038650482);
}

use std::collections::hash_map;

fn solve_p1(contents: String) -> i64 {
    let mut stones: Vec<i64> = contents
        .split(" ")
        .map(|s| {
            s.trim().parse::<i64>().expect("Valid ints in the line")
        })
        .collect();
    let mut mem = HashMap::<i64, Vec<i64>>::new();
    for _ in 0..25 {
        let mut new_stones = Vec::<i64>::new();
        for stone in stones {
            if let hash_map::Entry::Vacant(e) = mem.entry(stone) {
                let mut n_stone = Vec::<i64>::new();
                if stone == 0 {
                    n_stone.push(1);
                } else if format!("{stone}").len() % 2 == 0 {
                    let stone_str = format!("{stone}");
                    let left = stone_str[0..stone_str.len()/2]
                        .parse::<i64>().unwrap();
                    let right = stone_str[stone_str.len()/2..]
                        .parse::<i64>().unwrap();
                    n_stone.push(left);
                    n_stone.push(right);
                } else {
                    n_stone.push(stone * 2024);
                }
                n_stone.iter().for_each(|s| new_stones.push(*s));
                e.insert(n_stone);
            } else {
                let n_stone = mem.get(&stone).expect("where did the precompute go");
                n_stone.iter().for_each(|s| new_stones.push(*s));
            }
        }
        stones = new_stones.clone();
    }
    stones.len() as i64
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_11.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 55312);
}
