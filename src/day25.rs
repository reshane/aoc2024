
pub fn solve() {
    let contents = std::fs::read_to_string("input_25.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
}

#[derive(Debug, Clone)]
enum Kl {
    Key(Vec<i64>),
    Lock(Vec<i64>),
}

fn parse_kl(raw: &str) -> Kl {
    let split = raw.lines().collect::<Vec<&str>>();
    let mut hs: Vec<i64> = vec![0; split[0].len()];
    for line in split.iter() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                hs[i] += 1;
            }
        }
    }
    if &split[0][0..1] == "#" {
        return Kl::Key(hs);
    }
    Kl::Lock(hs)
}

fn parse_input(contents: String) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let kls = contents.split("\n\n").collect::<Vec<&str>>();
    let mut keys = vec![];
    let mut locks = vec![];
    for kl in kls.iter() {
        match parse_kl(kl) {
            Kl::Key(k) => keys.push(k),
            Kl::Lock(l) => locks.push(l),
        }
    }
    (keys, locks)
}

fn solve_p1(contents: String) -> i64 {
    let (keys, locks) = parse_input(contents);
    let mut total = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            let mut valid = true;
            for i in 0..key.len() {
                if 7 - (key[i] + lock[i]) < 0 {
                    valid = false;
                }
            }
            if valid {
                total += 1;
            }
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_25.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 3);
}

