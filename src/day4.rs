use std::collections::HashMap;

pub fn solve() {
    let contents = std::fs::read_to_string("input_4.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
    println!("secret 3rd part: {}", solve_nothing(contents));
}

// here is where I misunderstood the prompt
// This counts the number of XMAS sequences
// in any configuration...
//      Ex.
//          .X.A.
//          ..M.S
// This would be detected as a valid XMAS
// Cool, but completely useless here
fn solve_nothing(contents: String) -> i64 {
    let (xes, map) = parse_input(contents);
    // println!("{xes:?}");
    let mut total = 0;
    for start in xes {
        let mut queue = Vec::<(usize, (i64, i64))>::new();
        let ms: Vec<(i64,i64)> = map.get(&start).unwrap().clone();
        // println!("{start:?}: {ms:?}");
        ms.iter().for_each(|m| { queue.push((1, *m)) });
        // println!("{queue:?}");
        // for each neighbor, push it onto the queue with depth + 1
        while queue.len() > 0 {
            let current = queue.remove(0);
            if current.0 == 3 {
                // we have an S that started with an X, increment the total
                total += 1;
                // println!("Found XMAS");
            } else {
                let ns = map.get(&current.1).unwrap().clone();
                ns.iter().for_each(|n| { queue.push((current.0 + 1, *n)) });
                // println!("{queue:?}");
            }
        }
    }
    total
}

#[test]
fn test_sample_nothing() {
    let input = vec![
        ".X.A.",
        "..M.S",
        "....A",
        "XSA.M",
        ".M..X",
    ];
    let result = solve_nothing(input.join("\n"));
    println!("{result}");
    assert!(result == 3);
}

fn solve_p2(contents: String) -> i64 {
    let lines: Vec<&str> = contents.lines().collect();
    let mut total = 0;
    for i in 1..lines.len()-1 {
        let line = lines[i];
        // println!("{line}");
        for j in 1..line.len()-1 {
            let current_val = &line[j..j+1];
            // println!("{current_val}");
            if current_val == "A" {
                let jl: usize = (j as i64 - 1).try_into().unwrap();
                let il: usize = (i as i64 - 1).try_into().unwrap();
                let tl = &lines[il][jl..j];
                let br = &lines[i+1][j+1..j+2];
                let tlbr = (br == "M" || br == "S") && (tl == "M" || tl == "S") && br != tl;
                let tr = &lines[il][j+1..j+2];
                let bl = &lines[i+1][jl..j];
                let trbl = (bl == "M" || bl == "S") && (tr == "M" || tr == "S") && bl != tr;
                if tlbr && trbl {
                    total += 1;
                }
            }
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_4.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 9);
}

fn parse_input(contents: String) -> (Vec<(i64, i64)>, HashMap<(i64, i64), Vec<(i64, i64)>>) {
    let mut map = HashMap::<(i64, i64), Vec<(i64, i64)>>::new();
    let mut xes = Vec::<(i64, i64)>::new();
    // println!("{contents}");
    let lines: Vec<&str> = contents.lines().collect();
    for (i, line) in lines.clone().into_iter().enumerate() {
        for j in 0..line.len() {
            let current_val = &line[j..j+1];
            let current_pos = (j as i64, i as i64);
            if current_val == "X" {
                xes.push(current_pos);
            }
            let mut neighbors = Vec::<(i64, i64)>::new();
            let x_s: usize = (j as i64 - 1).try_into().unwrap_or(0);
            let y_s: usize = (i as i64 - 1).try_into().unwrap_or(0);
            // println!("{current_val} at {current_pos:?}");
            for x in x_s..j+2 {
                for y in y_s..i+2 {
                    if y < lines.len() && x < lines[y].len() {
                        if x != j || y != i {
                            // println!("{:?}", (x,y));
                            let neighbor_val = &lines[y][x..x+1];
                            match (current_val, neighbor_val) {
                                ("X", "M") | ("M", "A") | ("A", "S") => {
                                    // println!("{:?} at {:?} -> {:?} at {:?}", current_val, (j,i), neighbor_val, (x,y));
                                    neighbors.push((x as i64, y as i64));
                                },
                                (_, _) => {},
                            }
                        }
                    }
                }
            }
            map.insert(current_pos, neighbors);
        }
    }
    (xes, map)
}

fn solve_p1(contents: String) -> i64 {
    // only straight lines :facepalm:
    // expand out in 8 directions from each X
    // println!("{contents}");
    let (xes, map) = parse_input(contents.clone());
    let mut total = 0;
    for start in xes {
        // expand in all directions by adding one 
        // println!("checking: {start:?}");
        for dx in -1..2 {
            for dy in -1..2 {
                if dx != 0 || dy != 0 {
                    let mut current = start;
                    let mut seq = vec![current];
                    let mut count = 0;
                    while count < 3 {
                        let neighbors = map.get(&current).unwrap();
                        if neighbors.contains(&(current.0+dx, current.1+dy)) {
                            // println!("{:?} exists", (current.0+dx, current.1+dy));
                            current = (current.0+dx, current.1+dy);
                            seq.push(current);
                            count += 1;
                        } else {
                            // println!("{:?} doesnt exist", (current.0+dx, current.1+dy));
                            break;
                        }
                    }
                    if count == 3 {
                        // println!("{seq:?}");
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_4.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 18);
}
