use std::fs;
use std::collections::HashMap;

pub fn solve() {
    let contents = fs::read_to_string("input_1.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let mut freq = HashMap::<i64, i64>::new();
    let (mut left, right) = parse_input(contents);
    for e in right {
        if freq.contains_key(&e) {
            freq.insert(e, freq.get(&e).unwrap() + 1);
        } else {
            freq.insert(e, 1);
        }
    }
    let mut i = 0;
    while i < left.len() {
        left[i] = left[i] * freq.get(&left[i]).unwrap_or(&0);
        i += 1;
    }
    left.into_iter().sum()
}

#[test]
fn test_sample_2() {
    let contents = fs::read_to_string("sample_1.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    assert!(result == 31);
}

fn parse_input(contents: String) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec::<i64> = vec![];
    let mut right: Vec::<i64> = vec![];
    for line in contents.lines() {
        if line.len() > 0 {
            let line: Vec<&str> = line.split(" ").filter(|c| { c.parse::<i64>().is_ok() }).collect();
            left.push(line[0].parse::<i64>().unwrap());
            right.push(line[1].parse::<i64>().unwrap());
        }
    }

    (left, right)
}

fn solve_p1(contents: String) -> i64 {
    let (mut left, mut right) = parse_input(contents);
    let mut diffs = Vec::<i64>::new();
    left.sort();
    right.sort();
    let mut i = 0;
    while i < left.len() {
        let diff = (left[i] - right[i]).abs();
        diffs.push(diff);
        i += 1;
    }

    diffs.into_iter().sum()
}

#[test]
fn test_sample_1() {
    let contents = fs::read_to_string("sample_1.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    assert!(result == 11);
}
