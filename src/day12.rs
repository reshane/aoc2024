
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input_12.txt").expect("WHERE IS THE FILE");
    let start = std::time::Instant::now();
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
    println!("total execution time: {}ms", start.elapsed().as_millis());
}


fn solve_p2(contents: String) -> i64 {
    let (map, diags) = parse_input(&contents);
    let mut visited_global = HashSet::<Point>::new();
    let mut total = 0;
    for p in map.keys() {
        if !visited_global.contains(p) {
            // bfs, explore all nodes neighboring this one
            // incrementing area by each node explored
            // add to perimiter by 4 - node.nbors.len
            let mut queue = Vec::<Point>::new();
            queue.push(*p);
            let mut section_a = 0_i64;
            let mut sides = 0_i64;
            while !queue.is_empty() {
                // println!("{queue:?}");
                let curr = queue.remove(0);
                let nbors = map.get(&curr).expect("Where are the neighbors");
                let d_nbors = diags.get(&curr).expect("Where are the diagonal neighbors");
                section_a += 1;
                // if a node has opposing neighbors, it doesn't count towards a corner
                // if a node has two neighbors diagonal to eachother, it counts towards one corner
                // if a node has one neighbor, it counts towards 3 corners
                // a node missing any diagonal neighbors, counts as a corner for each missing
                // neighbor
                // if a node has 0 neighbors, it has 4 corners
                sides += match nbors.len() {
                    0 => 4_i64,
                    1 => 2_i64,
                    2 => {
                        if (nbors.iter().any(|(d,_)| *d == Dir::Up) &&
                            nbors.iter().any(|(d,_)| *d == Dir::Down)) ||
                            (nbors.iter().any(|(d,_)| *d == Dir::Left) &&
                            nbors.iter().any(|(d,_)| *d == Dir::Right)) {
                            // none
                            0_i64
                        } else {
                            // we have two neighbors diagonal to eachother
                            // if we have 0 diag neighbors, 2
                            // else, 1
                            // X...
                            // .CX. => 2 corners
                            // .X..
                            //
                            // X...
                            // .CX. => 1 corners
                            // .XX.
                            // 
                            // we need to get the diag between two neighbors
                            // C = (0,0)
                            // X1 = (1,0) LR
                            // X2 = (0,1) UD
                            // (UD.0, LR.1)  = (1, 1)
                            let (_, ud) = nbors.iter()
                                .find(|(d,_)| *d == Dir::Up || *d == Dir::Down).expect("Need uppy-downy neighbor");
                            let (_, lr) = nbors.iter()
                                .find(|(d,_)| *d == Dir::Left || *d == Dir::Right).expect("Need lefty-righty neighbor");
                            let d_n = (lr.0, ud.1);
                            if d_nbors.iter().any(|d| *d == d_n) {
                                // we have a diagonal neighbor present
                                1_i64
                            } else {
                                2_i64
                            }
                        }

                    },
                    3 => {
                        // if diags surrounded by nbor nodes are empty, this is a corner
                        let mut dc = 2_i64;
                        for d in d_nbors {
                            if d.1 > curr.1 {
                                if d.0 > curr.0 {
                                    // bottom right corner, needs right & bottom neighbor
                                    if nbors.iter().any(|(d,_)| { *d == Dir::Down }) &&
                                        nbors.iter().any(|(d,_)| { *d == Dir::Right }) {
                                        dc -= 1;
                                    }
                                } else {
                                    // bottom left corner, needs left & bottom neighbor
                                    if nbors.iter().any(|(d,_)| { *d == Dir::Down }) &&
                                        nbors.iter().any(|(d,_)| { *d == Dir::Left }) {
                                        dc -= 1;
                                    }
                                }
                            } else {
                                if d.0 > curr.0 {
                                    // top right corner, needs right & top neighbor
                                    if nbors.iter().any(|(d,_)| { *d == Dir::Up }) &&
                                        nbors.iter().any(|(d,_)| { *d == Dir::Right }) {
                                        dc -= 1;
                                    }
                                } else {
                                    // top left corner, needs left & top neighbor
                                    if nbors.iter().any(|(d,_)| { *d == Dir::Up }) &&
                                        nbors.iter().any(|(d,_)| { *d == Dir::Left }) {
                                        dc -= 1;
                                    }
                                }
                            }
                        }
                        assert!(dc >= 0);
                        dc
                    },
                    4 => {
                        assert!(d_nbors.len() < 5);
                        4_i64 - d_nbors.len() as i64
                    }
                    _ => unreachable!("Inconceivable!"),
                };
                assert!(sides > -1);
                visited_global.insert(curr);
                nbors.iter().for_each(|(_, n)| {
                    if !visited_global.contains(n) && !queue.contains(n) {
                        queue.push(*n);
                    }
                });
            }
            assert!(sides > 0);
            total += section_a * sides;
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_12.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 80);
}

type Point = (i64, i64);
#[derive(Eq, Hash, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(contents: &str) -> (HashMap<Point, Vec<(Dir, Point)>>, HashMap<Point, Vec<Point>>){
    let mut map = HashMap::<Point, Vec<(Dir, Point)>>::new();
    let mut diags = HashMap::<Point, Vec<Point>>::new();
    let lines = contents.lines().collect::<Vec<&str>>();
    let y_bound = lines.len();
    for (y, line) in lines.iter().enumerate() {
        let cs = line.chars().collect::<Vec<char>>();
        let x_bound = cs.len();
        for (x, c) in cs.iter().enumerate() {
            let mut nbors = vec![];
            // above
            if y > 0 && c.to_string() == lines[y-1][x..x+1] {
                // println!("{} -> {}", c.to_string(), &lines[y-1][x..x+1]);
                nbors.push((Dir::Up, (x as i64, y as i64 - 1)));
            }
            // below
            if y < y_bound-1 && c.to_string() == lines[y+1][x..x+1] {
                // println!("{} -> {}", c.to_string(), &lines[y+1][x..x+1]);
                nbors.push((Dir::Down, (x as i64, y as i64 + 1)));
            }
            // left
            if x > 0 && *c == cs[x-1] {
                // println!("{} -> {}", c, cs[x-1]);
                nbors.push((Dir::Left, (x as i64 - 1, y as i64)));
            }
            // right
            if x < x_bound-1 && *c == cs[x+1] {
                // println!("{} -> {}", c, cs[x+1]);
                nbors.push((Dir::Right, (x as i64 + 1, y as i64)));
            }
            let mut d_nbors = vec![];
            // above
            if y > 0 {
                // left
                if x > 0 && c.to_string() == lines[y-1][x-1..x] {
                    d_nbors.push((x as i64 - 1, y as i64 - 1));
                }
                // right
                if x < x_bound-1 && c.to_string() == lines[y-1][x+1..x+2] {
                    d_nbors.push((x as i64 + 1, y as i64 - 1));
                }
            }
            // below
            if y < y_bound-1 {
                // left
                if x > 0 && c.to_string() == lines[y+1][x-1..x] {
                    d_nbors.push((x as i64 - 1, y as i64 + 1));
                }
                // right
                if x < x_bound-1 && c.to_string() == lines[y+1][x+1..x+2] {
                    d_nbors.push((x as i64 + 1, y as i64 + 1));
                }
            }
            assert!(d_nbors.len() < 5);
            map.insert((x as i64, y as i64), nbors);
            diags.insert((x as i64, y as i64), d_nbors);
        }
    }
    (map, diags)
}

fn solve_p1(contents: String) -> i64 {
    let (map, _) = parse_input(&contents);
    let mut visited_global = HashSet::<Point>::new();
    let mut total = 0;
    for p in map.keys() {
        if !visited_global.contains(p) {
            // bfs, explore all nodes neighboring this one
            // incrementing area by each node explored
            // add to perimiter by 4 - node.nbors.len
            let mut queue = Vec::<Point>::new();
            queue.push(*p);
            let mut section_a = 0_i64;
            let mut section_p = 0_i64;
            while !queue.is_empty() {
                // println!("{queue:?}");
                let curr = queue.remove(0);
                let nbors = map.get(&curr).expect("Where are the neighbors");
                section_a += 1;
                section_p += 4 - nbors.len() as i64;
                visited_global.insert(curr);
                nbors.iter().for_each(|n| {
                    if !visited_global.contains(&n.1) && !queue.contains(&n.1) {
                        queue.push(n.1);
                    }
                });
            }
            // println!("{section_a}, {section_p}");
            total += section_a * section_p;
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_12.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 140);
}