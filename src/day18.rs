
pub fn solve() {
    let contents = std::fs::read_to_string("input_18.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

type Pos = (i64, i64);

fn parse_input(contents: String) -> Vec<Pos> {
    contents.lines()
        .filter_map(|line| { line.split_once(",") })
        .filter_map(|(x_str, y_str)| {
            let m = (x_str.parse::<i64>(), y_str.parse::<i64>());
            match m {
                (Ok(x),Ok(y)) => Some((x,y)),
                _ => None,
            }
        })
        .collect::<Vec<Pos>>()
}

fn solve_path_for_n(corrupt: HashSet<Pos>, bounds: Pos) -> i64 {
    // println!("{corrupt:?}");
    let (x_bound, y_bound) = bounds;
    let end_node = (x_bound, y_bound);
    let dims = (0..=x_bound).collect::<Vec<i64>>().into_iter().map(|x| {
            (0..=y_bound).collect::<Vec<i64>>().into_iter().map(|y| {
                (x,y)
            }).collect::<HashSet<Pos>>()
        }).collect::<Vec<HashSet<Pos>>>()
        .into_iter()
        .flatten().collect::<HashSet<Pos>>();

    let mut visited = HashMap::<Pos, usize>::new();
    let mut queue = VecDeque::<(Pos, usize)>::new();
    queue.push_back(((0,0), 0));
    'q: while let Some((pos, len)) = queue.pop_front() {
        // if pos is in corrupt, we can't go there, ignore it
        if corrupt.contains(&pos) {
            continue 'q;
        }
        // if we have visited this node, ignore it
        let queue_nbors;
        if let Some(prev_len) = visited.get(&pos) {
            queue_nbors = len < *prev_len;
            visited.insert(pos, std::cmp::min(len, *prev_len));
        } else {
            visited.insert(pos, len);
            queue_nbors = true;
        }

        if pos != end_node && queue_nbors {
            if dims.contains(&(pos.0-1, pos.1)) {
                queue.push_back(((pos.0-1, pos.1), len+1));
            }
            if dims.contains(&(pos.0+1, pos.1)) {
                queue.push_back(((pos.0+1, pos.1), len+1));
            }
            if dims.contains(&(pos.0, pos.1-1)) {
                queue.push_back(((pos.0, pos.1-1), len+1));
            }
            if dims.contains(&(pos.0, pos.1+1)) {
                queue.push_back(((pos.0, pos.1+1), len+1));
            }
        }
    }
    *visited.get(&end_node).unwrap_or(&0) as i64
}

fn solve_p1(contents: String) -> i64 {
    let corruptable = parse_input(contents);
    let corrupt = corruptable.into_iter().take(1024).collect::<HashSet<Pos>>();
    solve_path_for_n(corrupt, (70, 70))
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_18.txt").expect("WHERE IS THE FILE");
    let corruptable = parse_input(contents);
    let corrupt = (&corruptable[0..12]).to_vec().into_iter().collect::<HashSet<Pos>>();
    let result = solve_path_for_n(corrupt, (6, 6));
    println!("{result}");
    assert!(result == 22);
}

#[allow(dead_code)]
fn min_unsolvable(contents: String, bounds: Pos) -> String {
    // binary search for 0
    // in the corrupt input
    let corruptable = parse_input(contents);
    let mut n = 1;
    while n < corruptable.len() {
        let corrupt = &corruptable[0..=n];
        // println!("{corrupt:?}");
        let corrupt = HashSet::from_iter(corrupt.to_vec().into_iter());

        let res = solve_path_for_n(corrupt, bounds);
        // println!("{n}: {:?} -> {}", corruptable[n], res);

        if res == 0 {
            break;
        }
        n += 1;
    };
    let ans = corruptable[n];
    format!("{},{}", ans.0, ans.1)
}

fn min_unsolvable_bin_s(contents: String, bounds: Pos) -> String {
    // binary search for 0
    // in the corrupt input
    let corruptable = parse_input(contents);
    let mut n = 0;
    let mut jmp = corruptable.len() / 2;
    let mut mem = HashMap::<usize, i64>::new();
    while jmp > 0 {
        let corrupt = &corruptable[0..=n];
        let corrupt = HashSet::from_iter(corrupt.to_vec().into_iter());

        let res = solve_path_for_n(corrupt, bounds);
        mem.insert(n, res);

        if res == 0 {
            n -= jmp;
        } else {
            n += jmp;
        }
        jmp /= 2;
    };
    let ans = corruptable[n];

    format!("{},{}", ans.0, ans.1)
}

fn solve_p2(contents: String) -> String {
    min_unsolvable_bin_s(contents, (70,70))
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_18.txt").expect("WHERE IS THE FILE");
    let result = min_unsolvable(contents, (6,6));
    println!("{result}");
    assert!(result == "6,1".to_string());
}

#[test]
fn test_sample_2c() {
    let contents = std::fs::read_to_string("sample_18.txt").expect("WHERE IS THE FILE");
    let result = min_unsolvable_bin_s(contents, (6,6));
    println!("{result}");
    assert!(result == "6,1".to_string());
}

#[test]
fn test_sample_2b() {
    let contents = std::fs::read_to_string("sample_18.txt").expect("WHERE IS THE FILE");
    let corruptable = parse_input(contents);
    let corrupt = (&corruptable[0..21]).to_vec();
    println!("{corrupt:?}");
    let corrupt = corrupt.into_iter().collect::<HashSet<Pos>>();
    let result = solve_path_for_n(corrupt, (6, 6));
    println!("{result}");
    assert!(result == 0);
}

#[test]
fn test_sample_2d() {
    let contents = std::fs::read_to_string("input_18.txt").expect("WHERE IS THE FILE");
    let corruptable = parse_input(contents);
    let corrupt = (&corruptable[0..=2916]).to_vec();
    println!("{corrupt:?}");
    let corrupt = corrupt.into_iter().collect::<HashSet<Pos>>();
    let result = solve_path_for_n(corrupt, (70, 70));
    println!("{result}");
    assert!(result == 0);
}
