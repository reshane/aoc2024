
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input_19.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn parse_input(contents: String) -> Result<(Vec<String>, Vec<String>), ()> {
    if let Some((tokens, patterns)) = contents.split_once("\n\n") {
        let tokens = tokens.split(",")
            .map(|t| { t.trim().to_string() })
            .collect::<Vec<String>>();
        let patterns = patterns.lines()
            .map(|p| { p.trim().to_string() })
            .collect::<Vec<String>>();
        return Ok((tokens, patterns));
    }
    Err(())
}


fn solve_p1(contents: String) -> i64 {
    let (tokens, patterns) = parse_input(contents).unwrap();
    let mut total = 0;
    for pattern in patterns.iter() {
        // iterate through the pattern
        // put substrings on the queue
        let mut valid = false;
        let mut queue = BinaryHeap::<usize>::new();
        let mut visited = HashSet::<usize>::new();
        queue.push(0);
        while let Some(idx) = queue.pop() {
            if idx == pattern.len() {
                valid = true;
                break;
            }
            let mut acheived = HashSet::<usize>::new();
            for tok in tokens.iter() {
                // if one of them fits pattern[idx..], queue idx + tok.len()
                // a, aa, b, c
                // aabc
                //
                // idx = 0
                // a fits, queue idx + 1 = 1
                // aa fits, queue idx + 2 = 2
                if idx + tok.len() <= pattern.len() && *tok == pattern[idx..idx+tok.len()] {
                    if !visited.contains(&(idx+tok.len())) {
                        acheived.insert(idx+tok.len());
                    }
                }
            }
            acheived.iter().for_each(|a| { queue.push(*a) });
            visited.insert(idx);
        }
        if valid {
            total += 1;
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_19.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 6);
}

#[test]
fn test_sample_1b() {
    let contents = std::fs::read_to_string("sample_19b.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 0);
}

fn solve_p2(contents: String) -> i64 {
    let (tokens, patterns) = parse_input(contents).unwrap();
    let mut total = 0;
    for pattern in patterns.iter() {
        // iterate through the pattern
        // store the number of hits on any given position
        // each index holds the number of ways
        // to get to that index
        let mut dp = vec![0; pattern.len()+1];
        dp[0] = 1;
        let mut i = 0;
        while i < pattern.len() {
            let idx = i;
            for tok in tokens.iter() {
                if idx + tok.len() <= pattern.len() && *tok == pattern[idx..idx+tok.len()] {
                    dp[idx+tok.len()] += dp[idx];
                }
            }
            i += 1;
        }
        total += dp[pattern.len()];
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_19.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 16);
}

