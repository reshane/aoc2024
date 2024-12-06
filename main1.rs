use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
            }
            _ => {
                println!("NO IMPLEMENTATION!");
            },
        }
    } else {
        let contents = std::fs::read_to_string("input_6.txt").expect("WHERE IS THE FILE");
        println!("part 1: {}", solve_p1(contents.clone()));
        println!("part 2: {}", solve_p2(contents));
    }
}

