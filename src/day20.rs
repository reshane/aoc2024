
pub fn solve() {
    let contents = std::fs::read_to_string("input_20.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashMap;
use std::collections::VecDeque;

type Point = (i64, i64);

fn parse_input(contents: String) -> ((Point, Point), HashMap<Point, i64>) {
    let mut track = HashMap::<Point, i64>::new();
    let mut start = (-1,-1);
    let mut end = (-1,-1);

    contents.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let p_curr = (y as i64, x as i64);
                    track.insert(p_curr, match c {
                        '#' => 0,
                        '.' => 1,
                        'S' => {
                            start = p_curr;
                            1
                        },
                        'E' => {
                            end = p_curr;
                            1
                        },
                        _ => panic!("unrecognized character {c}"),
                    });
                });
        });

    ((start, end), track)
}

fn solve_p1(contents: String) -> i64 {
    let ((start, end), track) = parse_input(contents);
    solve_n_cheat(2, 100, &start, &end, &track)
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_20.txt").expect("WHERE IS THE FILE");
    let ((start, end), track) = parse_input(contents);
    let result = solve_n_cheat(2, 1, &start, &end, &track);
    println!("{result}");
    assert!(result == 44);
}

fn solve_p2(contents: String) -> i64 {
    let ((start, end), track) = parse_input(contents);
    solve_n_cheat(20, 100, &start, &end, &track)
}

fn solve_n_cheat(cheat_len: usize, lim: usize, start: &Point, end: &Point, track: &HashMap<Point, i64>) -> i64 {
    // flood fill from end to start
    // to get a distance map from the end for each node
    let mut d_map = HashMap::<Point, usize>::new();
    let mut q = VecDeque::<(Point, usize)>::new();
    q.push_back((*start, 0));
    while let Some((curr, len)) = q.pop_front() {
        if d_map.contains_key(&curr) {
            continue;
        }

        d_map.insert(curr, len);

        if curr == *end {
            break;
        }
        
        if let Some(&1) = track.get(&(curr.0+1,curr.1)) {
            q.push_back(((curr.0+1,curr.1),len+1));
        }
        if let Some(&1) = track.get(&(curr.0-1,curr.1)) {
            q.push_back(((curr.0-1,curr.1),len+1));
        }
        if let Some(&1) = track.get(&(curr.0,curr.1+1)) {
            q.push_back(((curr.0,curr.1+1),len+1));
        }
        if let Some(&1) = track.get(&(curr.0,curr.1-1)) {
            q.push_back(((curr.0,curr.1-1),len+1));
        }
    }

    let d_map = d_map.into_iter().collect::<Vec<(Point, usize)>>();
    let mut total = 0;
    let mut i = 0;
    while i < d_map.len()-1 {
        for j in i+1..d_map.len() {
            let (p1, c1) = d_map[i];
            let (p2, c2) = d_map[j];
            let c_len = (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as usize;
            if c_len <= cheat_len {
                let diff = c1.abs_diff(c2) - c_len;
                if diff >= lim {
                    total += 1;
                }
            }
        }
        i += 1;
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_20.txt").expect("WHERE IS THE FILE");
    let ((start, end), track) = parse_input(contents);
    let result = solve_n_cheat(6, 50, &start, &end, &track);
    println!("{result}");
    assert!(result == 46);
}
