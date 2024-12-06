use std::collections::HashSet;
use std::collections::HashMap;

pub fn solve() {
    let contents = std::fs::read_to_string("input_6.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

// How do we speed this thing up?
// maintain two hashmaps <i64, Vec<(i64, i64)>>
//      obstacles x coordinate -> obstacles
//      obstacles y coordinate -> obstacles
// this would speed up our search for potentially cycle generating positions
// use these hashmaps to build out a hashset of pcgps 

fn solve_p2(contents: String) -> i64 {
    // find the path of the guard
    let mut guard = parse_input(contents);
    let guard_start = guard.clone();
    let mut done = false;
    let mut cycling;
    while !done {
        (done, cycling) = guard.step();
        assert!(cycling == false);
    }
    // if placing an obstacle in the path creates a cycle, increment
    let mut total = 0;
    for p in guard.visited {
        // place an obstacle at p
        let mut test_guard = guard_start.clone();
        test_guard.obstacles.push(p);
        let mut done = false;
        let mut cycling;
        while !done {
            (done, cycling) = test_guard.step();
            if cycling {
                done = true;
                total += 1;
            }
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_6.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 6);
}

#[test]
fn test_sample_2b() {
    let input = vec![
        "...#......",
        ".........#",
        ".....#....",
        ".........#",
        "..........",
        ".....^....",
        "..........",
        "..........",
        "........#.",
        "..........",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 2);
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
struct Guard {
    pos: (i64, i64),
    dir: (i64, i64),
    bounds: (i64, i64),
    obstacles: Vec<(i64, i64)>,
    path_len: i64,
    visited: HashSet<(i64, i64)>,
    path: HashMap<(i64, i64), Vec<(i64, i64)>>,
    cycles: Vec<(i64, i64)>,
}

fn parse_input(contents: String) -> Guard {
    let mut guard: Guard = Guard::default();
    guard.path_len = 1;
    let mut obstacles: Vec<(i64, i64)> = vec![];
    contents.lines().enumerate().for_each(|(i, line)| {
        guard.bounds.1 += 1;
        guard.bounds.0 = line.len() as i64;
        line.chars().enumerate().for_each(|(j, c)| {
            match c {
                '.' => {},
                '#' => {
                    obstacles.push((j as i64, i as i64));
                },
                '^' => {
                    guard.pos = (j as i64, i as i64);
                    guard.dir = (0 as i64, -1 as i64);
                },
                '>' => {
                    guard.pos = (j as i64, i as i64);
                    guard.dir = (1 as i64, 0 as i64);
                },
                'v' => {
                    guard.pos = (j as i64, i as i64);
                    guard.dir = (0 as i64, 1 as i64);
                },
                '<' => {
                    guard.pos = (j as i64, i as i64);
                    guard.dir = (-1 as i64, 0 as i64);
                },
                _ => unreachable!(),
            }
        });
    });
    guard.obstacles = obstacles;
    guard.visited.insert(guard.pos);
    guard
}

impl Guard {
    fn step(&mut self) -> (bool, bool) {
        let mut potential_bumps: Vec<(i64, i64)> = vec![];
        match self.dir {
            (0, 1) | (0, -1) => {
                // find obstacle matching x
                self.obstacles.iter().for_each(|o| {
                    if self.pos.0 == o.0 {
                        // println!("{o:?}");
                        if (self.dir.1 < 0 && o.1 < self.pos.1) || 
                            (self.dir.1 > 0 && o.1 > self.pos.1) {
                            potential_bumps.push(*o);
                        }
                    }
                });
            },
            (1, 0) | (-1, 0) => {
                // find obstacle matching y
                self.obstacles.iter().for_each(|o| {
                    if self.pos.1 == o.1 {
                        if (self.dir.0 < 0 && o.0 < self.pos.0) || 
                            (self.dir.0 > 0 && o.0 > self.pos.0) {
                            potential_bumps.push(*o);
                        }
                    }
                });
            }
            _ => unreachable!(),
        }
        // println!("pb: {potential_bumps:?}");
        if potential_bumps.len() == 0 {
            // we are going to hit the wall
            match self.dir {
                (0,1) => { 
                    self.path_len += self.bounds.1 - self.pos.1; 
                    let path_start = self.pos.1;
                    let path_end = self.bounds.1 - self.dir.1;
                    let mut i = path_start;
                    while i != path_end && i > 0 {
                        i += self.dir.1;
                        let p = (self.pos.0, i);
                        /*
                        if self.obstacles.iter().any(|o| {
                            // if anything is colinear with the right side of the guard
                            // p + dir is a potential cycle creator
                            // here we are moving down
                            // so if there is any o such that px > ox && py == oy
                            // println!("leaving: {p:?} -> {o:?}: {}", o.0 < p.0 && o.1 == p.1);
                            o.0 < p.0 && o.1 == p.1
                        }) {
                            self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                        }
                        */
                        self.visited.insert(p);
                    }
                },
                (0,-1) => {
                    self.path_len += self.bounds.1; 
                    let path_start = self.pos.1;
                    let path_end = self.bounds.1 - self.dir.1;
                    let mut i = path_start;
                    while i != path_end && i > 0 {
                        i += self.dir.1;
                        let p = (self.pos.0, i);
                        /*
                        if self.obstacles.iter().any(|o| {
                            // if anything is colinear with the right side of the guard
                            // p + dir is a potential cycle creator
                            // here we are moving up
                            // so if there is any o such that px < ox && py == oy
                            // println!("leaving: {p:?} -> {o:?}: {}", o.0 > p.0 && o.1 == p.1);
                            o.0 > p.0 && o.1 == p.1
                        }) {
                            self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                        }
                        */
                        self.visited.insert(p);
                    }
                },
                (1,0) => {
                    self.path_len += self.bounds.0 - self.pos.0; 
                    let path_start = self.pos.0;
                    let path_end = self.bounds.0 - self.dir.0;
                    let mut i = path_start;
                    while i != path_end && i > 0 {
                        i += self.dir.0;
                        let p = (i, self.pos.1);
                        /*
                        if self.obstacles.iter().any(|o| {
                            // if anything is colinear with the right side of the guard
                            // p + dir is a potential cycle creator
                            // here we are moving right
                            // so if there is any o such that py > oy && px == ox
                            // println!("leaving: {p:?} -> {o:?}: {}",
                                // o.1 < p.1 && o.0 == p.0);
                            o.1 > p.1 && o.0 == p.0
                        }) {
                            self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                        }
                        */
                        self.visited.insert(p);
                    }
                },
                (-1,0) => {
                    self.path_len += self.bounds.0;
                    let path_start = self.pos.0;
                    let path_end = self.bounds.0 - self.dir.0;
                    let mut i = path_start;
                    while i != path_end && i > 0 {
                        i += self.dir.0;
                        let p = (i, self.pos.1);
                        /*
                        if self.obstacles.iter().any(|o| {
                            // if anything is colinear with the right side of the guard
                            // p + dir is a potential cycle creator
                            // here we are moving left
                            // so if there is any o such that py < oy && px == ox
                            // println!("leaving: {p:?} -> {o:?}: {}",
                                // o.1 > p.1 && o.0 == p.0);
                            o.1 < p.1 && o.0 == p.0
                        }) {
                            self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                        }
                        */
                        self.visited.insert(p);
                    }
                },
                _ => unreachable!(),
            }
            return (true, false);
        }
        potential_bumps.sort_by(|a, b| {
            let x1 = self.pos.0 - a.0;
            let y1 = self.pos.1 - a.1;
            let x2 = self.pos.0 - b.0;
            let y2 = self.pos.1 - b.1;
            let dist1 = x1.abs() + y1.abs();
            let dist2 = x2.abs() + y2.abs();
            dist1.cmp(&dist2)
        });
        // the closest point should be the first in the list
        let next_obs = potential_bumps[0];
        // println!("{next_obs:?}");
        // (4,6) -> (4,1)
        // (4-4, 6-1) = 0 + 5
        let mut repeated = false;
        if self.path.contains_key(&next_obs) {
            let mut dirs = self.path.get(&next_obs).expect("WHERE IS THE OBSTACLE").clone();
            if dirs.contains(&self.dir) {
                repeated = true;
            }
            dirs.push(self.dir);
            self.path.insert(next_obs, dirs);
        } else {
            self.path.insert(next_obs, vec![self.dir]);
        }
        // println!("{:?}", self.path);
        match self.dir {
            (0,1) | (0,-1) => { 
                self.path_len += (next_obs.1 - self.pos.1).abs(); 
                let path_start = self.pos.1;
                let path_end = next_obs.1 - self.dir.1;
                let mut i = path_start;
                while i != path_end {
                    i += self.dir.1;
                    // if the y matches
                    // and the next direction matches
                    // and visited does not contain
                    let p = (self.pos.0, i);
                    /*
                    if self.obstacles.iter().any(|o| {
                        // if anything is colinear with the right side of the guard
                        // p + dir is a potential cycle creator
                        // here we are moving up, down
                        // up: so if there is any o such that px < ox && py == oy
                        // down: so if there is any o such that px > ox && py == oy
                        if self.dir.1 == -1 {
                            // println!("on path: {p:?} -> {o:?}: {}",
                                // o.0 > p.0 && o.1 == p.1);
                            o.0 > p.0 && o.1 == p.1
                        } else {
                            // println!("on path: {p:?} -> {o:?}: {}",
                                // o.0 < p.0 && o.1 == p.1);
                            o.0 < p.0 && o.1 == p.1
                        }
                    }) {
                        self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                    }
                    */
                    self.visited.insert(p);
                }
            },
            (1,0) | (-1,0) => {
                self.path_len += (next_obs.0 - self.pos.0).abs();
                let path_start = self.pos.0;
                let path_end = next_obs.0 - self.dir.0;
                let mut i = path_start;
                while i != path_end {
                    i += self.dir.0;
                    let p = (i, self.pos.1);
                    /*
                    if self.obstacles.iter().any(|o| {
                        // if anything is colinear with the right side of the guard
                        // p + dir is a potential cycle creator
                        // here we are moving left, right
                        // left: so if there is any o such that py < oy && px == ox
                        // right: so if there is any o such that py > oy && px == ox
                        if self.dir.0 == -1 {
                            // println!("on path: {p:?} -> {o:?}: {}",
                                // o.1 > p.1 && o.0 == p.0);
                            o.1 < p.1 && o.0 == p.0
                        } else {
                            // on path moving right
                            // x has to be the same
                            // py > oy
                            // println!("on path: {p:?} -> {o:?}: {}",
                                // o.1 < p.1 && o.0 == p.0);
                            o.1 > p.1 && o.0 == p.0
                        }
                    }) {
                        self.cycles.push((p.0+self.dir.0, p.1+self.dir.1));
                    }
                    */
                    self.visited.insert(p);
                }
            },
            _ => unreachable!(),
        }
        self.pos = (next_obs.0 - self.dir.0, next_obs.1 - self.dir.1);
        self.dir = self.next_dir();
        (false, repeated)
    }

    fn next_dir(&self) -> (i64, i64) {
        match self.dir {
            (0,1) => (-1,0),
            (0,-1) => (1,0),
            (1,0) => (0,1),
            (-1,0) => (0,-1),
            _ => unreachable!(),
        }
    }
}

fn solve_p1(contents: String) -> i64 {
    // println!("{contents}");
    let mut guard = parse_input(contents);
    // println!("{guard:?}");
    let mut done = false;
    let mut cycling;
    while !done {
        (done, cycling) = guard.step();
        assert!(cycling == false);
    }
    guard.visited.len() as i64
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_6.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 41);
}
