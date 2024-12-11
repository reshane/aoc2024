
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input_5.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents.clone()));
}

fn solve_p2(contents: String) -> i64 {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let deps = parse_deps(split[0]);
    let raw_ords = split[1];

    let mut total = 0;
    for ord in raw_ords.lines() {

        let (mut ord, expected) = parse_ord(ord, &deps);

        if !expected.is_empty() {
            // we need to re-order them and then take the middle one
            if false {
                // check definition for a cool topo sort that is useless here
                let stack = reorder(&ord, &deps);
                println!("{stack:?}");
            }
            reorder_sort(&mut ord, &deps);
            total += ord[ord.len()/2];
        }
    }
    total
}

fn reorder_sort(ord: &mut [i64], deps: &HashMap<i64, Vec<i64>>) {
    ord.sort_by(|a, b| {
        if let Some(a_deps) = deps.get(a) {
            // if a depends on b, b must appear ahead of  a
            // so a is less than b
            if a_deps.contains(b) {
                return std::cmp::Ordering::Less;
            }
        }
        if let Some(b_deps) = deps.get(b) {
            // the other way round
            if b_deps.contains(a) {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
}

fn reorder(ord: &Vec<i64>, deps: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    // this sort implementation doesn't work for the problem :(
    // but does produce a vector sorted according to the topology
    let mut stack: Vec<i64> = vec![];
    for o in ord {
        // if ord contains a dependent of o
        // push the dependent onto the stack
        // if the stack already contains dep(o), dont push
        // then push o
        if let Some(o_dep) = deps.get(o) {
            let mut idx = o_dep.len();
            loop {
                idx -= 1;
                let d = o_dep[idx];
                if !stack.contains(&d) && ord.contains(&d) {
                    stack.push(d);
                }
                if idx == 0 {
                    break;
                }
            }
        }
        if !stack.contains(o) {
            stack.push(*o);
        }
    }
    stack
}

// Function for parsing the dependencies of each value
// generates the datastructure which stores our topology
fn parse_deps(raw_deps: &str) -> HashMap<i64, Vec<i64>> {
    let mut deps = HashMap::<i64, Vec<i64>>::new();
    // parse dependencies
    for line in raw_deps.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let dependency = parts[0].parse::<i64>().unwrap();
        let dependent = parts[1].parse::<i64>().unwrap();
        if deps.contains_key(&dependent) {
            let mut new_deps = deps.get(&dependent).unwrap().clone();
            new_deps.push(dependency);
            deps.insert(dependent, new_deps);
        } else {
            deps.insert(dependent, vec![dependency]);
        }
    }
    deps
}

// Parse a line of input and generate list of values expected but not found
// based on the topology
fn parse_ord(raw_ord: &str, deps: &HashMap<i64, Vec<i64>>) -> (Vec<i64>, HashSet<i64>) {
    // parse
    let ord: Vec<i64> = raw_ord.split(",")
        .map(|o| { o.parse::<i64>().unwrap() })
        .collect();

    // process
    let mut expected = HashSet::<i64>::new();
    let mut i = ord.len();
    loop {
        i -= 1;
        let o = ord[i];
        if expected.contains(&o) {
            expected.remove(&o);
        }
        if let Some(dep) = deps.get(&o) {
            dep.iter().for_each(|d| {
                if ord.contains(d) {
                    expected.insert(*d);
                }
            });
        }
        if i == 0 {
            break;
        }
    }
    (ord, expected)
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_5.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 123);
}

fn solve_p1(contents: String) -> i64 {
    let split: Vec<&str> = contents.split("\n\n").collect();
    let deps = parse_deps(split[0]);
    let raw_ords = split[1];
    let mut total = 0;
    for ord in raw_ords.lines() {

        let (ord, expected) = parse_ord(ord, &deps);

        if expected.is_empty() {
           total += ord[ord.len()/2]; 
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_5.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 143);
}
