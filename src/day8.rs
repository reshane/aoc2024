use std::collections::HashMap;
use std::collections::HashSet;

type Point = (i64, i64);

pub fn solve() {
    let contents = std::fs::read_to_string("input_8.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    // println!("{contents}");
    let ((width, height), freqs) = parse_input(contents);
    // println!("{freqs:?}");
    let mut antinodes = HashSet::<Point>::new();
    for f in freqs.keys() {
        let nodes: Vec<Point> = freqs.get(f).unwrap().clone();
        let mut i = 0;
        while i < nodes.len()-1 {
            let mut j = i + 1;
            while j < nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                antinodes.insert(a);
                antinodes.insert(b);
                let d = (a.0 - b.0, a.1 - b.1);
                // a + d and b - d
                let mut k = 1_i64;
                loop {
                    let ad = (a.0 + (d.0 * k), a.1 + (d.1 * k));
                    let bd = (b.0 - (d.0 * k), b.1 - (d.1 * k));
                    // println!("{ad:?}");
                    // println!("{bd:?}");
                    let a_ib = ad.0 >= 0 && ad.0 < width && ad.1 >= 0 && ad.1 < height;
                    if a_ib {
                        // we have a point here
                        antinodes.insert(ad);
                    }
                    let b_ib = bd.0 >= 0 && bd.0 < width && bd.1 >= 0 && bd.1 < height;
                    if b_ib {
                        antinodes.insert(bd);
                    }
                    if !a_ib && !b_ib {
                        break;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
    }
    antinodes.len() as i64
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_8.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 34);
}

fn parse_input(contents: String) -> (Point, HashMap<String, Vec<Point>>) {
    let mut nodes = HashMap::<String, Vec<Point>>::new();
    for (y, line) in contents.lines().enumerate() {
        for x in 0..line.len() {
            let freq = &line[x..x+1];
            match freq {
                "." => {},
                _ => {
                    if nodes.contains_key(freq) {
                        let mut ns = nodes.get(freq).unwrap().clone();
                        ns.push((x as i64, y as i64));
                        nodes.insert(freq.to_string(), ns.clone());
                    } else {
                        nodes.insert(freq.to_string(), vec![(x as i64, y as i64)]);
                    }
                },
            }
        }
    }
    let lns = contents.lines().collect::<Vec<&str>>();
    ((lns.len() as i64, lns[0].len() as i64), nodes)
}

fn solve_p1(contents: String) -> i64 {
    // println!("{contents}");
    let ((width, height), freqs) = parse_input(contents);
    // println!("{freqs:?}");
    let mut antinodes = HashSet::<Point>::new();
    for f in freqs.keys() {
        let nodes: Vec<Point> = freqs.get(f).unwrap().clone();
        let mut i = 0;
        while i < nodes.len()-1 {
            let mut j = i + 1;
            while j < nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let d = (a.0 - b.0, a.1 - b.1);
                // a + d and b - d
                let ad = (a.0 + d.0, a.1 + d.1);
                let bd = (b.0 - d.0, b.1 - d.1);
                // println!("{ad:?}");
                // println!("{bd:?}");
                if ad.0 >= 0 && ad.0 < width && ad.1 >= 0 && ad.1 < height {
                    // we have a point here
                    antinodes.insert(ad);
                }
                if bd.0 >= 0 && bd.0 < width && bd.1 >= 0 && bd.1 < height {
                    antinodes.insert(bd);
                }
                j += 1;
            }
            i += 1;
        }
    }
    antinodes.len() as i64
}

#[test]
fn test_sample_1b() {
    let contents = std::fs::read_to_string("sample_8b.txt").expect("WHERE IS THE FILE");
    // 3,1 6,7
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 2);
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_8.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 14);
}

