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
    let mut guard = Guard::from_string(contents);
    let guard_start = guard.clone();
    guard.gather_info = true;
    let mut done = false;
    let mut cycling;
    while !done {
        (done, cycling) = guard.step();
        assert!(!cycling);
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

type Point = (i64, i64);

#[derive(Default, Debug, Clone)]
struct Guard {
    gather_info: bool,
    pos: Point,
    dir: Point,
    bounds: Point,
    obstacles: Vec<Point>,
    path_len: i64,
    visited: HashSet<Point>,
    path: HashMap<Point, Vec<Point>>,
    cycles: Vec<(Point, Point, Point)>,
}



impl Guard {

    fn from_string(contents: String) -> Guard {
        let mut guard: Guard = Guard { path_len: 1, ..Default::default() };
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
                        guard.dir = (0_i64, -1_i64);
                    },
                    '>' => {
                        guard.pos = (j as i64, i as i64);
                        guard.dir = (1_i64, 0_i64);
                    },
                    'v' => {
                        guard.pos = (j as i64, i as i64);
                        guard.dir = (0_i64, 1_i64);
                    },
                    '<' => {
                        guard.pos = (j as i64, i as i64);
                        guard.dir = (-1_i64, 0_i64);
                    },
                    _ => unreachable!(),
                }
            });
        });
        guard.obstacles = obstacles;
        guard.visited.insert(guard.pos);
        guard
    }

    fn eval_potential_obs(&mut self, p: (i64, i64)) {
        // if anything is colinear with the right side of the guard
        // p + dir is a potential cycle creator
        if self.gather_info {
            let potentially_cyclical = match self.dir {
                (0,1) => {
                    // here we are moving down
                    // so if there is any o such that px > ox && py == oy
                    self.obstacles.iter().any(|o| {
                        o.0 < p.0 && o.1 == p.1
                    })
                },
                (0,-1) => {
                    self.obstacles.iter().any(|o| {
                        o.0 > p.0 && o.1 == p.1
                    })
                },
                (1,0) => {
                    self.obstacles.iter().any(|o| {
                        o.1 > p.1 && o.0 == p.0
                    })
                },
                (-1,0) => {
                    self.obstacles.iter().any(|o| {
                        o.1 < p.1 && o.0 == p.0
                    })
                },
                _ => unreachable!(),
            };
            let pc = (p.0+self.dir.0, p.1+self.dir.1);
            if potentially_cyclical && 
                !self.cycles.contains(&(pc, p, self.dir)) &&
                !self.obstacles.contains(&pc)
            {
                self.cycles.push((pc, p, self.dir));
            }
        }
    }
    
    fn run_into_wall(&mut self) {
        // we are going to hit the wall
        // add points to the visited set until a bound is reached
        match self.dir {
            (0,1) => { self.path_len += self.bounds.1 - self.pos.1; },
            (0,-1) => { self.path_len += self.bounds.1; },
            (1,0) => { self.path_len += self.bounds.0 - self.pos.0; },
            (-1,0) => { self.path_len += self.bounds.0; },
            _ => unreachable!(),
        }
        match self.dir {
            (0,1) | (0,-1) => { 
                let path_start = self.pos.1;
                let path_end = self.bounds.1 - self.dir.1;
                let mut i = path_start;
                while i != path_end && i > 0 {
                    i += self.dir.1;
                    let p = (self.pos.0, i);
                    self.eval_potential_obs(p);
                    self.visited.insert(p);
                }
            },
            (1,0) | (-1,0) => {
                let path_start = self.pos.0;
                let path_end = self.bounds.0 - self.dir.0;
                let mut i = path_start;
                while i != path_end && i > 0 {
                    i += self.dir.0;
                    let p = (i, self.pos.1);
                    self.eval_potential_obs(p);
                    self.visited.insert(p);
                }
            },
            _ => unreachable!(),
        }
    }

    fn run_into_object(&mut self, next_obs: (i64, i64)) {
        match self.dir {
            (0,1) | (0,-1) => { 
                self.path_len += (next_obs.1 - self.pos.1).abs(); 
                let path_start = self.pos.1;
                let path_end = next_obs.1 - self.dir.1;
                let mut i = path_start;
                while i != path_end {
                    i += self.dir.1;
                    let p = (self.pos.0, i);
                    self.eval_potential_obs(p);
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
                    self.eval_potential_obs(p);
                    self.visited.insert(p);
                }
            },
            _ => unreachable!(),
        }
        self.pos = (next_obs.0 - self.dir.0, next_obs.1 - self.dir.1);
        self.dir = self.next_dir();
    }

    fn step(&mut self) -> (bool, bool) {
        let mut potential_bumps: Vec<(i64, i64)> = vec![];
        match self.dir {
            (0, 1) | (0, -1) => {
                // find obstacle matching x
                self.obstacles.iter().for_each(|o| {
                    if self.pos.0 == o.0 && 
                        ( (self.dir.1 < 0 && o.1 < self.pos.1) || 
                        (self.dir.1 > 0 && o.1 > self.pos.1) ) {
                        potential_bumps.push(*o);
                    }
                });
            },
            (1, 0) | (-1, 0) => {
                // find obstacle matching y
                self.obstacles.iter().for_each(|o| {
                    if self.pos.1 == o.1 && 
                        ( (self.dir.0 < 0 && o.0 < self.pos.0) || 
                        (self.dir.0 > 0 && o.0 > self.pos.0) ) {
                        potential_bumps.push(*o);
                    }
                });
            }
            _ => unreachable!(),
        }
        if potential_bumps.is_empty() {
            // construct the visited set
            self.run_into_wall();
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
        let mut repeated = false;

        // construct the path & detect any cycles
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

        // construct the visited set
        self.run_into_object(next_obs);

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
    let mut guard = Guard::from_string(contents);
    let mut done = false;
    let mut cycling;
    while !done {
        (done, cycling) = guard.step();
        assert!(!cycling);
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
