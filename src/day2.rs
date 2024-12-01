
pub fn solve() {
    let contents = std::fs::read_to_string("input_2.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let records = parse_input(contents);
    let mut safe_count = 0;
    for record in records {
        if record.iter().enumerate().any(|(i, _)| {
            let mut record = record.clone();
            record.remove(i);
            let diffs: Vec<i64> = record
                .windows(2)
                .map(|window| { window[0] - window[1] })
                .collect();
            diffs.iter().all(|&x| { x < 4 && x > 0 }) ||
                diffs.iter().all(|&x| { x >-4 && x < 0 })
        }) {
            safe_count += 1;
        }
    }
    safe_count
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_2.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    assert!(result == 4);
}

fn parse_input(contents: String) -> Vec<Vec<i64>> {
    let mut records: Vec<Vec<i64>> = vec![];
    for line in contents.lines() {
        records.push(line.split(" ").map(|c| { c.parse::<i64>().unwrap() }).collect());
    }
    records
}

fn solve_p1(contents: String) -> i64 {
    let records = parse_input(contents);
    let mut safe_count = 0;
    for record in records {
        let diffs: Vec<i64> = record
            .windows(2)
            .map(|window| { window[0] - window[1] })
            .collect();
        if diffs.iter().all(|&x| { x < 4 && x > 0 }) ||
            diffs.iter().all(|&x| { x >-4 && x < 0 }) {
            safe_count += 1;
        }
    }
    safe_count
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_2.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 2);
}
