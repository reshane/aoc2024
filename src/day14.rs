
pub fn solve() {
    let contents = std::fs::read_to_string("input_14.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}


use std::collections::HashSet;

fn solve_p2(contents: String) -> i64 {
    let mut robots = parse_input(contents);
    let width = 101;
    let height = 103;
    let mut seconds = 0;
    let step = 1;

    's: loop {
        let mut positions = HashSet::<(i64, i64)>::new();
        for robot in robots.iter_mut() {
            let mut new_x = (robot.pos.0 + (robot.vel.0 * step)) % width;
            let mut new_y = (robot.pos.1 + (robot.vel.1 * step)) % height;

            if new_x < 0 {
                new_x += width;
            }
            if new_y < 0 {
                new_y += height;
            }
            robot.pos.0 = new_x;
            robot.pos.1 = new_y;

            positions.insert(robot.pos);
        }
        seconds += step;
        if positions.len() == robots.len() {
            display(&robots);
            break 's;
        }
    }
    seconds
}

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn display(robots: &[Robot]) {
    for line in 0..101 {
        for col in 0..103 {
            let count = robots.iter().fold(0, |acc, robot| {
                if robot.pos.0 == col && robot.pos.1 == line {
                    acc + 1
                } else {
                    acc
                }
            });
            print!(
                "{}",
                if count == 0 {
                    String::from(".")
                } else {
                    count.to_string()
                }
            );
        }
        println!();
    }
}

fn parse_input(contents: String) -> Vec<Robot> {
    let mut robots = vec![];
    for line in contents.lines().filter(|l| { !l.is_empty() }) {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let p_str = parts[0];
        let v_str = parts[1];
        let px = p_str[p_str.find("=").unwrap()+1..
                        p_str.find(",").unwrap()]
                        .parse::<i64>().unwrap();
        let py = p_str[p_str.find(",").unwrap()+1..]
                        .parse::<i64>().unwrap();
        let vx = v_str[v_str.find("=").unwrap()+1..
                        v_str.find(",").unwrap()]
                        .parse::<i64>().unwrap();
        let vy = v_str[v_str.find(",").unwrap()+1..]
                        .parse::<i64>().unwrap();
        robots.push(Robot {
            pos: (px, py),
            vel: (vx, vy),
        });
    }
    robots
}

fn solve_p1(contents: String) -> i64 {
    let robots = parse_input(contents);
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let width = 101;
    let height = 103;
    let step = 100;
    let x_quad = (width - 1) / 2;
    let y_quad = (height - 1) / 2;
    // println!("{},{}", x_quad, y_quad);
    for mut robot in robots.into_iter() {
        let mut new_x = (robot.pos.0 + (robot.vel.0 * step)) % width;
        let mut new_y = (robot.pos.1 + (robot.vel.1 * step)) % height;

        if new_x < 0 {
            new_x += width;
        }
        if new_y < 0 {
            new_y += height;
        }

        robot.pos.0 = new_x;
        robot.pos.1 = new_y;

        // println!("{},{}", robot.pos.0, robot.pos.1);
        if robot.pos.0 != x_quad && robot.pos.1 != y_quad {
            if robot.pos.0 > x_quad {
                if robot.pos.1 > y_quad {
                    q4 += 1;
                } else {
                    q3 += 1;
                }
            } else {
                if robot.pos.1 > y_quad {
                    q2 += 1;
                } else {
                    q1 += 1;
                }
            }
        }
    }
    // println!("{q1} * {q2} * {q3} * {q4}");
    q1 * q2 * q3 * q4
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_14.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 21);
}
