
pub fn solve() {
    let contents = std::fs::read_to_string("input_16.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let maze = parse_input(contents);
    maze.best_path_len()
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_16.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 45);
}

#[test]
fn test_sample_2b() {
    let contents = std::fs::read_to_string("sample_16b.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 64);
}

#[test]
fn test_sample_2c() {
    let contents = std::fs::read_to_string("sample_16c.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 18);
}

use std::fmt::Display;
use std::fmt::Formatter;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Point = (i64, i64);

#[derive(Debug, PartialEq)]
enum MazeTile {
    Path,
    Wall,
    Start,
    End,
}


impl Display for MazeTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MazeTile::Path => write!(f, "."),
            MazeTile::Wall => write!(f, "#"),
            MazeTile::Start => write!(f, "S"),
            MazeTile::End => write!(f, "E"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
enum Dir {
    North,
    South,
    #[default]
    East,
    West,
}

#[derive(Debug, Default)]
struct Reindeer {
    pos: Point,
    dir: Dir,
}

#[derive(Debug, Default)]
struct Maze {
    rd: Reindeer,
    mz: Vec<Vec<MazeTile>>,
    end: Point,
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.mz.len() {
            let row = &self.mz[y];
            row.iter().enumerate().for_each(|(x, t)| {
                if (x as i64, y as i64) == self.rd.pos {
                    match self.rd.dir {
                        Dir::North => { let _ = write!(f, "^"); },
                        Dir::South => { let _ = write!(f, "v"); },
                        Dir::East  => { let _ = write!(f, ">"); },
                        Dir::West  => { let _ = write!(f, "<"); },
                    }
                } else {
                    let _ = write!(f, "{}", t);
                }
            });
            let _ = writeln!(f);
        }
        Ok(())
    }
}

fn parse_input(contents: String) -> Maze {
    let mut rd_pos = Point::default();
    let mut end = Point::default();
    let mz = contents.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    match c {
                        'S' => {
                            rd_pos = (x as i64, y as i64);
                            Some(MazeTile::Start)
                        },
                        'E' => {
                            end = (x as i64, y as i64);
                            Some(MazeTile::End)
                        }
                        '#' => Some(MazeTile::Wall),
                        '.' => Some(MazeTile::Path),
                        _ => None,
                    }
                }).collect::<Vec<MazeTile>>()
        }).collect::<Vec<Vec<MazeTile>>>();
    Maze { 
        rd: Reindeer { 
            pos: rd_pos, 
            ..Default::default()
        },
        mz,
        end
    }
}

#[derive(Eq, PartialEq)]
struct StackFrame {
    pos: Point,
    dir: Dir,
    scr: i64,
    pth: Vec<Point>,
}

impl StackFrame {
    fn new(pos: Point, dir: Dir, scr: i64, pth: Vec<Point>) -> Self {
        Self {
            pos,
            dir,
            scr,
            pth,
        }
    }
}


impl Ord for StackFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        other.scr.cmp(&self.scr)
    }
}

impl PartialOrd for StackFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


impl Maze {
    fn best_path_len(&self) -> i64 {
        let path_len = self.solve();
        let mut path_nodes = HashSet::<Point>::new();
        let mut stack = BinaryHeap::<StackFrame>::new();
        let mut visited = HashMap::<(Point, Dir), i64>::new();
        stack.push(StackFrame {
            pos: self.rd.pos,
            dir: Dir::East,
            scr: 0,
            pth: vec![self.rd.pos] });
        while let Some(top) = stack.pop() {
            // if the top of the stack is the end
            // push everything from the stack into path_nodes
            if let Some(prev) = visited.get(&(top.pos, top.dir.clone())) {
                if top.scr > *prev {
                    continue;
                }
            } else {
                visited.insert((top.pos, top.dir.clone()), top.scr);
            }

            if top.pos == self.end && top.scr <= path_len {
                top.pth.iter().for_each(|n| { path_nodes.insert(*n); });
            }
            // if the top of the stack has neighbors that are not in visited
            // and they are < path_len away, push them
            // else pop the stack into visited
            let ln = (top.pos.0-1, top.pos.1);
            let rn = (top.pos.0+1, top.pos.1);
            let an = (top.pos.0, top.pos.1-1);
            let bn = (top.pos.0, top.pos.1+1);
            if ln.0 > 0 && self.mz[ln.1 as usize][ln.0 as usize] != MazeTile::Wall &&
                    top.dir != Dir::East {
                let mut sc = top.scr + 1;
                if top.dir != Dir::West {
                    sc += 1000;
                }
                if sc <= path_len {
                    let mut np = top.pth.clone();
                    np.push(ln);
                    stack.push(StackFrame::new(ln, Dir::West, sc, np));
                }
            }
            if (rn.0 as usize) < self.mz[rn.1 as usize].len() && 
                    self.mz[rn.1 as usize][rn.0 as usize] != MazeTile::Wall &&
                    top.dir != Dir::West {
                let mut sc = top.scr + 1;
                if top.dir != Dir::East {
                    sc += 1000;
                }
                if sc <= path_len {
                    let mut np = top.pth.clone();
                    np.push(rn);
                    stack.push(StackFrame::new(rn, Dir::East, sc, np));
                }
            }
            if an.1 > 0 && self.mz[an.1 as usize][an.0 as usize] != MazeTile::Wall &&
                    top.dir != Dir::South {
                let mut sc = top.scr + 1;
                if top.dir != Dir::North {
                    sc += 1000;
                }
                if sc <= path_len {
                    let mut np = top.pth.clone();
                    np.push(an);
                    stack.push(StackFrame::new(an, Dir::North, sc, np));
                }
            }
            if (bn.1 as usize) < self.mz.len() &&
                    self.mz[bn.1 as usize][bn.0 as usize] != MazeTile::Wall &&
                    top.dir != Dir::North {
                let mut sc = top.scr + 1;
                if top.dir != Dir::South {
                    sc += 1000;
                }
                if sc <= path_len {
                    let mut np = top.pth.clone();
                    np.push(bn);
                    stack.push(StackFrame::new(bn, Dir::South, sc, np));
                }
            }

        }
        path_nodes.len() as i64
    }

    fn solve(&self) -> i64 {
        let mut min_end = i64::MAX;
        let mut queue = Vec::<(Point, Dir, i64)>::new();
        let mut visited = HashMap::<Point, i64>::new();
        queue.push((self.rd.pos, self.rd.dir.clone(), 0));
        while !queue.is_empty() {
            let curr = queue.remove(0);
            if curr.1 != Dir::East && curr.0.0 > 0 {
                // explore tile to left
                let nbor = &self.mz[curr.0.1 as usize][(curr.0.0 - 1) as usize];
                let nbor_pos = (curr.0.0 - 1, curr.0.1);
                let mut n_score = curr.2 + 1;
                if curr.1 != Dir::West {
                    n_score += 1000;
                }
                match nbor {
                    MazeTile::End => {
                        min_end = std::cmp::min(n_score, min_end);
                    },
                    MazeTile::Path => {
                        if !visited.contains_key(&nbor_pos) {
                            // if we got here faster than before, push to queue and replace in hm
                            queue.push((nbor_pos, Dir::West, n_score));
                            visited.insert(nbor_pos, n_score);
                        } else {
                            let prev_score = visited.get(&nbor_pos).unwrap();
                            if n_score < *prev_score {
                                queue.push((nbor_pos, Dir::West, n_score));
                                visited.insert(nbor_pos, n_score);
                            }
                        }
                    },
                    MazeTile::Start | MazeTile::Wall => {},
                }
            }
            if curr.1 != Dir::West && 
                (curr.0.0 as usize) < self.mz[curr.0.1 as usize].len() {
                // explore tile to right
                let nbor = &self.mz[curr.0.1 as usize][(curr.0.0 + 1) as usize];
                let nbor_pos = (curr.0.0 + 1, curr.0.1);
                let mut n_score = curr.2 + 1;
                if curr.1 != Dir::East {
                    n_score += 1000;
                }
                match nbor {
                    MazeTile::End => {
                        min_end = std::cmp::min(n_score, min_end);
                    },
                    MazeTile::Path => {
                        if !visited.contains_key(&nbor_pos) {
                            // if we got here faster than before, push to queue and replace in hm
                            queue.push((nbor_pos, Dir::East, n_score));
                            visited.insert(nbor_pos, n_score);
                        } else {
                            let prev_score = visited.get(&nbor_pos).unwrap();
                            if n_score < *prev_score {
                                queue.push((nbor_pos, Dir::East, n_score));
                                visited.insert(nbor_pos, n_score);
                            }
                        }
                    },
                    MazeTile::Start | MazeTile::Wall => {},
                }
            }
            if curr.1 != Dir::North && 
                (curr.0.1 as usize) < self.mz.len() {
                // explore tile below
                let nbor = &self.mz[(curr.0.1 + 1) as usize][curr.0.0 as usize];
                let nbor_pos = (curr.0.0, curr.0.1 + 1);
                let mut n_score = curr.2 + 1;
                if curr.1 != Dir::South {
                    n_score += 1000;
                }
                match nbor {
                    MazeTile::End => {
                        min_end = std::cmp::min(n_score, min_end);
                    },
                    MazeTile::Path => {
                        if !visited.contains_key(&nbor_pos) {
                            // if we got here faster than before, push to queue and replace in hm
                            queue.push((nbor_pos, Dir::South, n_score));
                            visited.insert(nbor_pos, n_score);
                        } else {
                            let prev_score = visited.get(&nbor_pos).unwrap();
                            if n_score < *prev_score {
                                queue.push((nbor_pos, Dir::South, n_score));
                                visited.insert(nbor_pos, n_score);
                            }
                        }
                    },
                    MazeTile::Start | MazeTile::Wall => {},
                }
            }
            if curr.1 != Dir::South && curr.0.1 > 0 {
                // explore tile above
                let nbor = &self.mz[(curr.0.1 - 1) as usize][curr.0.0 as usize];
                let nbor_pos = (curr.0.0, curr.0.1 - 1);
                let mut n_score = curr.2 + 1;
                if curr.1 != Dir::North {
                    n_score += 1000;
                }
                match nbor {
                    MazeTile::End => {
                        min_end = std::cmp::min(n_score, min_end);
                    },
                    MazeTile::Path => {
                        if !visited.contains_key(&nbor_pos) {
                            // if we got here faster than before, push to queue and replace in hm
                            queue.push((nbor_pos, Dir::North, n_score));
                            visited.insert(nbor_pos, n_score);
                        } else {
                            let prev_score = visited.get(&nbor_pos).unwrap();
                            if n_score < *prev_score {
                                queue.push((nbor_pos, Dir::North, n_score));
                                visited.insert(nbor_pos, n_score);
                            }
                        }
                    },
                    MazeTile::Start | MazeTile::Wall => {},
                }
            }
        }
        min_end
    }
}

fn solve_p1(contents: String) -> i64 {
    let maze = parse_input(contents);
    maze.solve()
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_16.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 7036);
}

#[test]
fn test_sample_1b() {
    let contents = std::fs::read_to_string("sample_16b.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 11048);
}
