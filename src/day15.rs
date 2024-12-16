
pub fn solve() {
    let contents = std::fs::read_to_string("input_15.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashSet;
use std::fmt::Display;

type Pos = (i64, i64);

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
struct WWarehouse {
    dims: Pos,
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    moves: Vec<Dir>,
}


impl Display for WWarehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut y = 0;
        while y <= self.dims.1 {
            let mut x = 0;
            while x <= self.dims.0 {
                if self.walls.contains(&(x, y)) {
                    let _ = write!(f, "#");
                } else if self.boxes.contains(&(x, y)) {
                    let _ = write!(f, "[]");
                    x += 1;
                } else if self.robot == (x, y) {
                    let _ = write!(f, "@");
                } else {
                    let _ = write!(f, ".");
                }
                x += 1;
            }
            let _  = write!(f, "\n");
            y += 1;
        }
        if !self.moves.is_empty() {
            let _ = write!(f, "{:?}", self.moves[0]);
        }
        Ok(())
    }
}


fn w_parse_input(contents: String) -> WWarehouse {
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let map_str = split[0];
    let dir_str = split[1];

    let mut dims = (0, 0);

    let mut warehouse = WWarehouse::default();

    for (y, line) in map_str.lines().filter(|line| { !line.is_empty() }).enumerate() {
        dims.1 = y as i64;
        for (x, c) in line.chars().enumerate() {
            let x = x * 2;
            dims.0 = x as i64 + 1;
            match c {
                '#' => {
                    // wall
                    warehouse.walls.insert((x as i64, y as i64));
                    warehouse.walls.insert((x as i64 + 1, y as i64));
                },
                'O' => {
                    // box
                    warehouse.boxes.insert((x as i64, y as i64));
                },
                '@' => {
                    warehouse.robot = (x as i64, y as i64);
                },
                '.' => {},
                _ => {},
            }
        }
    }

    warehouse.dims = dims;

    dir_str.chars()
        .filter_map(|c| {
            match c {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            }
        })
        .for_each(|dir| { warehouse.moves.push(dir) });

    warehouse
}

impl WWarehouse {
    fn step(&mut self) {
        let dir = self.moves.remove(0);
        let dir_vec = match dir {
            Dir::Up => (0_i64, -1_i64),
            Dir::Down => (0_i64, 1_i64),
            Dir::Left => (-1_i64, 0_i64),
            Dir::Right => (1_i64, 0_i64),
        };
        match dir {
            Dir::Up | Dir::Down => {
                // start from robot + dir_vec
                // if we have a box, put it in the vector (box & box + 1x)
                // then for each box in the vector
                // explore box + dir_vec
                // if we encounter a wall, return
                // if the vector is empty, we have to add dir_vec to all the boxes
                // and the robot
                let mut queue = Vec::<Pos>::new();
                let next = (self.robot.0, self.robot.1 + dir_vec.1);
                if self.walls.contains(&next) {
                    return;
                }
                if let Some(bx) = self.boxes.get(&next) {
                    // there is a box directly above
                    queue.push(*bx);
                }
                if let Some(bx) = self.boxes.get(&(next.0 - 1, next.1)) {
                    // there is a box to the left
                    queue.push(*bx);
                }

                let mut visited = HashSet::<Pos>::new();

                while !queue.is_empty() {
                    let current = queue.remove(0);
                    let next = (current.0, current.1 + dir_vec.1);

                    if self.walls.contains(&next) {
                        return;
                    }
                    if self.walls.contains(&(next.0 + 1, next.1)) {
                        return;
                    }

                    if let Some(bx) = self.boxes.get(&next) {
                        // there is a box directly above
                        queue.push(*bx);
                    }
                    if let Some(bx) = self.boxes.get(&(next.0 - 1, next.1)) {
                        // there is a box to the left
                        queue.push(*bx);
                    }
                    if let Some(bx) = self.boxes.get(&(next.0 + 1, next.1)) {
                        // there is a box to the right
                        queue.push(*bx);
                    }

                    visited.insert(current);
                }

                visited.iter().for_each(|b| {
                    self.boxes.remove(&b);
                });
                visited.iter().for_each(|b| {
                    self.boxes.insert((b.0, b.1 + dir_vec.1));
                });
                self.robot.1 += dir_vec.1;
            },
            Dir::Right | Dir::Left => {
                let mut target = (self.robot.0 + dir_vec.0, self.robot.1);
                let robot_next_pos = target.clone();

                if dir == Dir::Left {
                    target.0 += dir_vec.0;
                }

                let mut boxes_to_move = Vec::<Pos>::new();
                while self.boxes.contains(&target) {
                    boxes_to_move.push(target);
                    target.0 += 2 * dir_vec.0;
                }

                let mut wall_test = target;
                if dir == Dir::Left {
                    wall_test.0 += 1;
                }

                if self.walls.contains(&wall_test) {
                    // there is a wall & we don't move
                    return;
                }

                boxes_to_move.iter().for_each(|b| {
                    self.boxes.remove(&b);
                });
                boxes_to_move.iter().for_each(|b| {
                    self.boxes.insert((b.0 + dir_vec.0, b.1));
                });
                
                self.robot = robot_next_pos;
            },
        }
    }
}

fn solve_p2(contents: String) -> i64 {
    let mut w_warehouse = w_parse_input(contents);
    // println!("{w_warehouse}");
    while !w_warehouse.moves.is_empty() {
        w_warehouse.step();
        // println!("{w_warehouse}");
    }
    w_warehouse.boxes
        .iter()
        .fold(0, |acc, b| {
            acc + (b.1 * 100) + b.0
        })
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_15.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 9021);
}

#[derive(Debug, Default)]
struct Warehouse {
    walls: HashSet<Pos>,
    boxes: HashSet<Pos>,
    robot: Pos,
    moves: Vec<Dir>,
}

fn parse_input(contents: String) -> Warehouse {
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let map_str = split[0];
    let dir_str = split[1];

    let mut warehouse = Warehouse::default();

    for (y, line) in map_str.lines().filter(|line| { !line.is_empty() }).enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    // wall
                    warehouse.walls.insert((x as i64, y as i64));
                },
                'O' => {
                    // box
                    warehouse.boxes.insert((x as i64, y as i64));
                },
                '@' => {
                    warehouse.robot = (x as i64, y as i64);
                },
                '.' => {},
                _ => {},
            }
        }
    }

    dir_str.chars()
        .filter_map(|c| {
            match c {
                '^' => Some(Dir::Up),
                'v' => Some(Dir::Down),
                '<' => Some(Dir::Left),
                '>' => Some(Dir::Right),
                _ => None,
            }
        })
        .for_each(|dir| { warehouse.moves.push(dir) });

    warehouse
}

impl Warehouse {
    fn step(&mut self) {
        let dir = self.moves.remove(0);
        let dir_vec = match dir {
            Dir::Up => (0_i64, -1_i64),
            Dir::Down => (0_i64, 1_i64),
            Dir::Left => (-1_i64, 0_i64),
            Dir::Right => (1_i64, 0_i64),
        };

        let mut target = (self.robot.0 + dir_vec.0, self.robot.1 + dir_vec.1);
        let robot_next_pos = target.clone();
        let mut boxes_to_move = Vec::<Pos>::new();

        // update target until it is not in the boxes
        while self.boxes.contains(&target) {
            boxes_to_move.push(target);
            target.1 += dir_vec.1;
            target.0 += dir_vec.0;
        }

        if self.walls.contains(&target) {
            // there is a wall & we don't move
            return;
        }

        // the vector of boxes are a vertical line of boxes
        // we can just move the first encountered
        // to the empty position after last (aka target)
        if boxes_to_move.len() > 0 {
            self.boxes.remove(&robot_next_pos);
            self.boxes.insert(target);
        }
        self.robot = robot_next_pos;
    }
}

fn solve_p1(contents: String) -> i64 {
    let mut warehouse = parse_input(contents);
    while !warehouse.moves.is_empty() {
        // println!("{warehouse:?}");
        warehouse.step();
    }
    // println!("{warehouse:?}");
    warehouse.boxes
        .iter()
        .fold(0, |acc, b| {
            acc + (b.1 * 100) + b.0
        })
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_15.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 10092);
}
