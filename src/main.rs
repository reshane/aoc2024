use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Too many args! pick a day");
    } else if args.len() == 2 {
        let day = args[1].clone().parse::<i64>().unwrap_or_else(|_err| {
            println!("provided argument is not a valid day");
            return 0;
        });
        match day {
            1 => {
                day1::solve();
            },
            2 => {
                day2::solve();
            },
            3 => {
                day3::solve();
            },
            4 => {
                day4::solve();
            },
            5 => {
                day5::solve();
            },
            6 => {
                day6::solve();
            },
            7 => {
                day7::solve();
            },
            8 => {
                day8::solve();
            },
            9 => {
                day9::solve();
            },
            10 => {
                day10::solve();
            },
            _ => {
                println!("NO IMPLEMENTATION!");
            },
        }
    } else {
        solve();
    }
}

pub fn solve() {
    let contents = std::fs::read_to_string("input_11.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(_contents: String) -> i64 {
    0
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_11.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 0);
}

fn solve_p1(_contents: String) -> i64 {
    0
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_11.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 0);
}
