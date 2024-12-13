
pub fn solve() {
    let contents = std::fs::read_to_string("input_13.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let eqs = parse_input(contents);
    let mut total = 0;
    for eq in eqs {

        let a = eq.0;
        let b = eq.1;
        let c = (eq.2.0 + 10000000000000, eq.2.1 + 10000000000000);

        // ax*a + bx*b = cx
        // ay*a + by*b = cy
        // 
        // b = (cy - ay*a) / by
        // a = (cx - bx*b) / ax
        // 
        // b = (cy - ay*((cx - bx*b) / ax)) / by
        // b = (cy - ((ay*cx - ay*bx*b) / ax)) / by
        // cy - b*by = (ay*cx - ay*bx*b) / ax
        // ax*cy - ax*b*by = ay*cx - ay*bx*b
        // ay*bx*b - ax*b*by = ay*cx - ax*cy
        // b*(ay*bx - ax*by) = ay*cx - ax*cy
        // b = (ay*cx - ax*cy) / (ay*bx - ax*by)
        // 
        // b = (cy - ay*cx) / (ax - ay*by)
        // a = (cx - bx*b) / ax

        let b_c = (a.1 * c.0 - a.0 * c.1) / (a.1 * b.0 - a.0 * b.1);
        let a_c = (c.0 - b.0 * b_c) / a.0;

        if a.0 * a_c + b.0 * b_c == c.0 &&
            a.1 * a_c + b.1 * b_c == c.1 {
            total += (3 * a_c) + b_c;
        }
    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_13.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 875318608908);
}

type V2 = (i64,i64);

fn parse_input(contents: String) -> Vec<(V2, V2, V2)> {
    contents.split("\n\n")
        .map(|lines| {
            let tups = lines
                .split("\n")
                .enumerate()
                .filter(|(_,line)| { !line.is_empty() })
                .map(|(i, line)| {
                if i == 2 {
                    let x = &line[line.find("X=").unwrap()+2..line.find(", Y").unwrap()];
                    let x = x.parse::<i64>().unwrap();
                    let y = &line[line.find("Y=").unwrap()+2..];
                    let y = y.parse::<i64>().unwrap();
                    (x, y)
                } else {
                    let x = &line[line.find("X+").unwrap()+2..line.find(", Y").unwrap()];
                    let x = x.parse::<i64>().unwrap();
                    let y = &line[line.find("Y+").unwrap()+2..];
                    let y = y.parse::<i64>().unwrap();
                    (x, y)
                }
            }).collect::<Vec<V2>>();
            (tups[0], tups[1], tups[2])
        })
        .collect::<Vec<(V2, V2, V2)>>()
}

fn solve_p1(contents: String) -> i64 {
    let eqs = parse_input(contents);
    let mut total = 0;
    'eq: for eq in eqs {
        // precompute all multiples up to 100 of the x value
        // of both a and b
        let a = eq.0;
        let b = eq.1;
        let c = eq.2;
        let mut ax_m = vec![a.0];
        let mut bx_m = vec![b.0];
        for i in 1..100 {
            ax_m.push(ax_m[i-1] + a.0);
            bx_m.push(bx_m[i-1] + b.0);
        }

        // if any combination of elements from a and b works for c.0
        // test out the multiples of their y values
        for (i, ax) in ax_m.iter().enumerate() {
            for (j, bx) in bx_m.iter().enumerate() {
                let i = i as i64 + 1;
                let j = j as i64 + 1;
                if ax + bx == c.0 {
                    if i * a.1 + j * b.1 == c.1 {
                        total += 3 * i + j as i64;
                        continue 'eq;
                    }
                }
            }
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_13.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 480);
}
