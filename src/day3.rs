
pub fn solve() {
    let contents = std::fs::read_to_string("input_3.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

pub fn solve_p1(contents: String) -> i64 {
    parse_mul(contents.as_str())
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_3a.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 161);
}

fn parse_mul(line: &str) -> i64 {
    let mut total = 0;
    let mut idx = 0;
    while let Some(idx_s) = line[idx..].find("mul(") {
        let line = &line[idx..]; // this way we don't have to add a bunch of indices
        if let (Some(idx_d), Some(idx_e)) = (line[idx_s..].find(","), line[idx_s..].find(")")) {
            let idx_d = idx_s + idx_d;
            let idx_e = idx_s + idx_e;
            if idx_d < idx_e && idx_s < idx_d {
                // idx_s+4..idx_d should be our first operand
                // idx_d+1..idx_e should be our next operand
                let op1 = line[idx_s+4..idx_d].parse::<i64>();
                let op2 = line[idx_d+1..idx_e].parse::<i64>();
                if op1.is_ok() && op2.is_ok() {
                    total += op1.unwrap() * op2.unwrap();
                }
            }
        }
        idx += idx_s + 4;
    }
    total
}

pub fn solve_p2(contents: String) -> i64 {
    let mut total: i64 = 0;
    let blocks: Vec<&str> = contents.split("don't()").collect();
    for (j, block) in blocks.into_iter().enumerate() {
        assert!(!block.contains("don't()"));
        if j == 0 {
            let line = block;
            total += parse_mul(line);
        } else if let Some(line_idx) = block.find("do()") {
            let line = &block[line_idx..];
            total += parse_mul(line);
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_3b.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 48);
}
