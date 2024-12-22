
pub fn solve() {
    let contents = std::fs::read_to_string("input_21.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashMap;
use std::collections::VecDeque;

const NUM_PAD: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DIR_PAD: [[u8; 3]; 2] = [
    [b' ', b'^', b'A'],
    [b'<', b'v', b'>'],
];

const DIRS: [(i32, i32); 4] = [
    ( 0, -1), // UP
    ( 0,  1), // DOWN
    (-1,  0), // LEFT
    ( 1,  0), // RIGHT
];

const DIR_TABLE: [u8; 4] = [
    b'^', // UP
    b'v', // DOWN
    b'<', // LEFT
    b'>', // RIGHT
];

fn find_shortest_paths(
    src: u8, dst: u8, pad: &[[u8; 3]],
    cache: &mut HashMap<(u8,u8,usize),Vec<Vec<u8>>>
) -> Vec<Vec<u8>> {
    if let Some(cached) = cache.get(&(src,dst,pad.len())) {
        return cached.to_vec();
    }
    let mut start = (0, 0);
    let mut end = (0, 0);
    pad.iter().enumerate().for_each(|(y,row)| {
        row.iter().enumerate().for_each(|(x, &c)| {
            if c == src {
                start = (x as i32, y as i32);
            }
            if c == dst {
                end = (x as i32, y as i32);
            }
        });
    });

    let mut dists = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((end.0, end.1, 0));
    while let Some((x, y, d)) = queue.pop_front() {
        if dists.contains_key(&(x,y)) {
            continue;
        }
        dists.insert((x, y), d);
        DIRS.iter().for_each(|(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < 3
                && ny >= 0
                && ny < pad.len() as i32 
                && pad[y as usize][x as usize] != b' ' {
                queue.push_back((nx, ny, d+1));
            }
        });
    }

    let mut stack: Vec<((i32,i32),Vec<u8>)> = vec![];
    let mut paths = vec![];
    stack.push((start, vec![]));
    while let Some(((x, y), mut p)) = stack.pop() {
        if (x, y) == end {
            p.extend(vec![b'A']);
            paths.push(p);
            continue;
        }
        DIRS.iter().enumerate().for_each(|(i, (dx, dy))| {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < 3
                && ny >= 0
                && ny < pad.len() as i32 
                && pad[y as usize][x as usize] != b' ' {
                if let Some(d) = dists.get(&(nx,ny)) {
                    if d < dists.get(&(x,y)).unwrap() {
                        let c = DIR_TABLE[i];
                        let mut p = p.clone();
                        p.push(c);
                        stack.push(((nx,ny),p));
                    }
                }
            }
        });
    }

    cache.insert((src,dst,pad.len()), paths.clone());

    paths
}

fn shortest_steps(
    start: u8,
    end: u8,
    level: usize,
    top: bool,
    cache: &mut HashMap<(u8, u8, usize), usize>,
    p_cache: &mut HashMap<(u8,u8,usize), Vec<Vec<u8>>>
) -> usize {
    if let Some(cached) = cache.get(&(start,end,level)) {
        return *cached;
    }
    let targets = find_shortest_paths(
        start, end,
        if top { &NUM_PAD } else { &DIR_PAD },
        p_cache
    );
    if level == 0 {
        return targets[0].len();
    }
    let mut total = usize::MAX;
    for target in targets.iter() {
        let mut result = 0;
        let mut current = b'A';
        for next in target.iter() {
            result += shortest_steps(
                current,
                *next,
                level-1,
                false,
                cache,
                p_cache
            );
            current = *next;
        }
        if result < total {
            total = result;
        }
    }
    cache.insert((start,end,level), total);
    total
}

fn solve_p1(contents: String) -> i64 {
    let mut total = 0;
    contents.lines().for_each(|line| {
        let pref_num = line[0..3].parse::<usize>().unwrap();
        let mut prefix = vec![b'A'];
        prefix.extend(line.as_bytes());
        let line = prefix;
        let mut sequence_len = 0;
        line
            .windows(2)
            .for_each(|bytes| {
                let src = bytes[0];
                let dst = bytes[1];
                sequence_len += shortest_steps(
                    src, dst, 2, true, &mut HashMap::new(), &mut HashMap::new()
                );
            });
        total += sequence_len * pref_num;
    });
    total as i64
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_21.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 126384);
}

fn solve_p2(contents: String) -> i64 {
    let mut total = 0;
    contents.lines().for_each(|line| {
        let pref_num = line[0..3].parse::<usize>().unwrap();
        let mut prefix = vec![b'A'];
        prefix.extend(line.as_bytes());
        let line = prefix;
        let mut sequence_len = 0;
        line
            .windows(2)
            .for_each(|bytes| {
                let src = bytes[0];
                let dst = bytes[1];
                sequence_len += shortest_steps(
                    src, dst, 25, true, &mut HashMap::new(), &mut HashMap::new()
                );
            });
        total += sequence_len * pref_num;
    });
    total as i64
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_21.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 154115708116294);
}
