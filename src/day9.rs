
pub fn solve() {
    let contents = std::fs::read_to_string("input_9.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    // push all the file blocks onto a stack
    // while the stack is not empty, try to put it
    // as far left as possible
    // sets of indices w/length
    //      one with files, one with empty blocks
    let mut total: i64 = 0;
    for line in contents.lines() {
        let mut empty: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
        let mut occupied: Vec<(usize, usize, usize)> = Vec::<(usize, usize, usize)>::new();
        let mut idx = 0;
        for i in 0..line.len() {
            let blocks = line[i..i+1].parse::<usize>().unwrap();
            if i % 2 == 0 {
                // file
                occupied.insert(0, (idx, blocks, i/2));
            } else {
                // empty
                empty.push((idx, blocks));
            }
            idx += blocks;
        }
        // println!("{empty:?}");
        // println!("{occupied:?}");
        for f_idx in 0..occupied.len() {
            let file = occupied[f_idx];
            // if there is an empty whose empty.0 is < file.1 && empty.1 >= file.2
            if let Some(target) = empty.clone().iter().find(|&&empty| { empty.0 < file.0 && empty.1 >= file.1 }) {
                // change the index of file, update the occupied entry
                // if there is an empty entry which touches the file, combine them
                // println!("Can move {} to {}: {:?} -> {:?}", file.2, target.0, file, target);
                // update the occupied
                occupied.remove(f_idx);
                occupied.push((target.0, file.1, file.2));
                // sort
                occupied.sort_by(|a, b| {
                    b.2.cmp(&a.2)
                });
                // update the empty
                empty.remove(empty.clone().iter().position(|t| t == target).unwrap());
                // target size - file size
                // target idx + file size
                empty.push((target.0+file.1, target.1-file.1));
                empty.push((file.0, file.1));
                empty.sort_by(|a, b| {
                    a.0.cmp(&b.0)
                });
                // println!("{empty:?}");
                // println!("{occupied:?}");
                // flatten overlapping empty spaces
                let mut i = 0;
                while i < empty.len()-1 {
                    let left = empty[i];
                    let right = empty[i+1];
                    if left.0+left.1 >= right.0 {
                        let len = right.0+right.1 - left.0;
                        empty[i] = (left.0, len);
                        empty.remove(i+1);
                    }
                    i += 1;
                }
                // println!("{empty:?}");
            }
        }

        total += occupied.iter().map(|(a, b, c)| {
            let mut acc = 0;
            let start: usize = *a;
            let end = *a + *b;
            for idx in start..end {
                acc += idx * c;
            }
            acc as i64
        }).sum::<i64>();

    }
    total
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_9.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 2858);
}

fn solve_p1(contents: String) -> i64 {
    const RADIX: u32 = 10;
    let mut total: i64 = 0;
    for line in contents.lines() {
        // characters alternate occupied & free
        let total_blocks = line.chars()
            .enumerate()
            .map(|(_,c)| { c.to_digit(RADIX).unwrap() as usize })
            .sum::<usize>();
        let file_blocks = line.chars()
            .enumerate()
            .filter(|(i,_)| { i % 2 == 0 })
            .map(|(_,c)| { c.to_digit(RADIX).unwrap() as usize })
            .sum::<usize>();
        let mut blocks: Vec<i64> = vec![-1; total_blocks];
        // println!("{file_blocks}");
        // println!("{total_blocks}");
        let mut idx = 0;
        line.chars().enumerate().for_each(|(i,c)| {
            let block_count: usize = c.to_digit(RADIX).unwrap().try_into().unwrap();
            for _j in 0..block_count {
                if i % 2 == 0 {
                    blocks[idx] = i as i64 / 2;
                } else {
                    blocks[idx] = -1;
                }
                idx += 1;
            }
        });
        // println!("{blocks:?}");
        let mut i = 0;
        let mut j = blocks.len()-1;
        while blocks[j] < 0 { j -= 1; }
        while i < file_blocks {
            if blocks[i] < 0 {
                blocks[i] = blocks[j];
                blocks[j] = -1;
                while blocks[j] < 0 { j -= 1; }
                // println!("{blocks:?}");
            }
            i += 1;
        }
        for k in 0..i {
            total += blocks[k] * k as i64;
        }
        // println!("{blocks:?}");
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_9.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 1928);
}
