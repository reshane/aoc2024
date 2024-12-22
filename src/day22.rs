
pub fn solve() {
    let contents = std::fs::read_to_string("input_22.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::ops::BitXor;
use std::collections::HashMap;
use std::collections::VecDeque;

const PRUNE: u64 = 16777216;

fn mix(s: u64, r: u64) -> u64 {
    s.bitxor(r)
}

fn prune(s: u64) -> u64 {
    s % PRUNE
}

fn calc_n(s: u64, n: u64) -> u64 {
    let mut prev = s;
    let mut next = 0;
    for _ in 0..n {
        next = prune(mix(prev, prev * 64));
        next = prune(mix(next, next / 32));
        next = prune(mix(next, next * 2048));
        prev = next;
    }
    next
}

fn next_secret(s: u64) -> u64 {
    let mut next = prune(mix(s, s * 64));
    next = prune(mix(next, next / 32));
    next = prune(mix(next, next * 2048));
    next
}

fn solve_p1(contents: String) -> i64 {
    contents.lines()
        .filter_map(|line| { line.parse::<usize>().ok() })
        .map(|s| {
            let res = calc_n(s as u64, 2000) as i64;
            // println!("{s}: {res}");
            res
        })
        .sum()
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_22.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 37327623);
}

type Seq = (i8, i8, i8, i8);

fn solve_p2(contents: String) -> i64 {
    // for every line
    // calculate each iteration of the secret
    // keep the price diff sequence for 4 up to current price
    // if that sequence has no entry, add the price for this buyer
    // if it has an entry, add to the sum, update buyer so we don't max
    // out each sequence regardless of position for each buyer
    let mut seq_sums = HashMap::<Seq, (usize, u64)>::new();
    let mut curr_seq = VecDeque::<i8>::new();
    contents.lines()
        .filter_map(|line| { line.parse::<u64>().ok() })
        .enumerate()
        .for_each(|(curr_buyer, s)| {
            let mut prev = s;
            for _ in 0..2000 {
                let curr = next_secret(prev);
                let diff = (curr % 10) as i8 - (prev % 10) as i8;
                curr_seq.push_back(diff);
                if curr_seq.len() == 4 {
                    let seq = (curr_seq[0], curr_seq[1], curr_seq[2], curr_seq[3]);
                    if let Some((buyer, bananas)) = seq_sums.get(&seq) {
                        if *buyer != curr_buyer {
                            seq_sums.insert(seq, (curr_buyer, *bananas + (curr % 10)));
                        }
                    } else {
                        seq_sums.insert(seq, (curr_buyer, curr % 10));
                    }
                    curr_seq.pop_front();
                }
                prev = curr;
            }
            curr_seq.clear();
        });

    seq_sums.into_iter()
        .map(|(_, v)| { v.1 })
        .max().unwrap() as i64
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_22.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 24);
}

#[test]
fn test_sample_2b() {
    let contents = std::fs::read_to_string("sample_22b.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 23);
}
