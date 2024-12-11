use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input_10.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let (trailheads, map) = parse_input(contents);
    // for each trailhead, bfs
    // every node popped off the queue with elevation = 9, increment counter
    let mut total: i64 = 0;
    for trailhead in trailheads {
        let mut queue = Vec::<Node>::new();
        queue.push(trailhead);
        while !queue.is_empty() {
            let curr = queue.remove(0);
            if curr.e == 9 {
                total += 1;
            } else {
                let nbors = map.get(&curr).unwrap();
                for nbor in nbors {
                    queue.push(nbor.clone());
                }
            }
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_10.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 81);
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Node {
    e: i64,
    x: i64,
    y: i64,
}

fn parse_input(contents: String) -> (Vec<Node>, HashMap<Node, Vec<Node>>) {
    let mut trailheads: Vec<Node> = vec![];
    let mut map = HashMap::<Node, Vec<Node>>::new();

    let contents: Vec<&str> = contents.split("\n")
        .filter(|l| !l.is_empty() )
        .collect();
    for y in 0..contents.len() {
        let line = contents[y];
        for x in 0..line.len() {
            let curr = &line[x..x+1].parse::<i64>().unwrap();
            let curr_node = Node { e: *curr, x: x as i64, y: y as i64 };
            if *curr == 0 {
                trailheads.push(curr_node.clone());
            }
            let mut nbors = Vec::<Node>::new();
            if y != 0 {
                // above
                let above = contents[y-1][x..x+1].parse::<i64>().unwrap();
                if  above == curr + 1 {
                    nbors.push(Node { e: above, x: x as i64, y: y as i64 - 1 });
                }
            }
            if y < contents.len() - 1 {
                // below
                let below = contents[y+1][x..x+1].parse::<i64>().unwrap();
                if below == curr + 1 {
                    nbors.push(Node { e: below, x: x as i64, y: y as i64 + 1 });
                }
            }
            if x != 0 {
                // left
                let left = line[x-1..x].parse::<i64>().unwrap();
                if left == curr + 1 {
                    nbors.push(Node { e: left, x: x as i64 - 1, y: y as i64 });
                }
            }
            if x < line.len() - 1 {
                // right
                let right = line[x+1..x+2].parse::<i64>().unwrap();
                if right == curr + 1 {
                    nbors.push(Node { e: right, x: x as i64 + 1, y: y as i64 });
                }
            }
            map.insert(curr_node, nbors);
        }
    }

    (trailheads, map)
}

fn solve_p1(contents: String) -> i64 {
    let (trailheads, map) = parse_input(contents);
    // for each trailhead, bfs
    // every node popped off the queue with elevation = 9, increment counter
    let mut total: i64 = 0;
    for trailhead in trailheads {
        let mut queue = Vec::<Node>::new();
        let mut summits = HashSet::<Node>::new();
        queue.push(trailhead);
        while !queue.is_empty() {
            let curr = queue.remove(0);
            if curr.e == 9 {
                if !summits.contains(&curr) {
                    total += 1;
                    summits.insert(curr);
                }
            } else {
                let nbors = map.get(&curr).unwrap();
                for nbor in nbors {
                    queue.push(nbor.clone());
                }
            }
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_10.txt").expect("WHERE IS THE FILE");
    println!("{contents}");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 36);
}

