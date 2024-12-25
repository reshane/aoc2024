
pub fn solve() {
    let contents = std::fs::read_to_string("input_24.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

const DEBUG: bool = false;

#[derive(Debug, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let raw = raw.trim();
        match raw {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct BinOp {
    lhs: String,
    op: Operation,
    rhs: String,
}

impl FromStr for BinOp {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let split = raw.split_whitespace().collect::<Vec<&str>>();
        Ok(Self {
            lhs: split[0].to_string(),
            op: Operation::from_str(split[1])?,
            rhs: split[2].to_string(),
        })
    }
}

#[derive(Debug, Clone)]
enum Def {
    Val(bool),
    Eq(BinOp),
}

#[derive(Debug, Clone)]
struct Emulator {
    mem: HashMap<String, Def>,
    cache: HashMap<String, bool>,
}

impl FromStr for Emulator {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let mut map = HashMap::new();
        if let Some((vals, defs)) = raw.split_once("\n\n") {
            defs.lines()
                .for_each(|line| {
                    if let Some((eq, var)) = line.split_once("->") {
                        if let Ok(eq) = BinOp::from_str(eq) {
                            map.insert(var.trim().to_string(), Def::Eq(eq));
                        }
                    }
                });
            vals.lines()
                .for_each(|line| {
                    if let Some((var, val)) = line.split_once(":") {
                        let val = val.trim();
                        let val = match val {
                            "1" => true,
                            "0" => false,
                            _ => panic!("Cannot parse value"),
                        };
                        map.insert(var.trim().to_string(), Def::Val(val));
                    }
                });

        }
        Ok(Self {
            mem: map,
            cache: HashMap::new(),
        })
    }
}

impl Emulator {

    fn query(&mut self, n: &String) -> Result<bool, ()> {
        if let Some(v) = self.cache.get(n) {
            return Ok(*v);
        }
        if let Some(d) = self.mem.clone().get(n) {
            // println!("{n} -> {d:?}");
            let v = self.eval(d)?;
            self.cache.insert(n.to_string(), v);
            return Ok(v);
        }
        Err(())
    }

    fn eval(&mut self, d: &Def) -> Result<bool, ()> {
        match d {
            Def::Val(v) => Ok(*v),
            Def::Eq(bop) => {
                let l = self.query(&bop.lhs)?;
                let r = self.query(&bop.rhs)?;
                match bop.op {
                    Operation::And => {
                        Ok(l && r)
                    },
                    Operation::Or => {
                        Ok(l || r)
                    },
                    Operation::Xor => {
                        Ok(l ^ r)
                    }
                }
            },
        }
    }

    fn walk(&self, n: &String, level: usize, max: usize) -> HashSet<String> {
        let mut children = HashSet::new();
        if level > max {
            return children
        }
        let node = self.mem.get(n).unwrap();
        if DEBUG {
            for _ in 0..level {
                print!(" ");
            }
            println!("{n}: {:?}", node);
        }
        if let Def::Eq(bop) = node {
            children.insert(n.to_string());
            let mut lc = self.walk(&bop.lhs.clone(), level+1, max);
            let mut rc = self.walk(&bop.rhs.clone(), level+1, max);
            lc.drain().for_each(|c| { children.insert(c); });
            rc.drain().for_each(|c| { children.insert(c); });
        }
        children
    }

}

fn compute_output(emu: &mut Emulator) -> Vec<u128> {
    let mut x = 0_u128;
    let mut y = 0_u128;
    let mut z = 0_u128;
    let keys = emu.mem.clone();
    let keys = keys.keys().collect::<Vec<&String>>();
    for name in keys {
        if &name[0..1] == "x" || &name[0..1] == "y" || &name[0..1] == "z" {
            let lshft = name[1..].parse::<usize>().unwrap();
            let mask = 1 << lshft;
            let val = emu.query(name).unwrap();
            if val {
                match &name[0..1] {
                    "x" => {
                        x |= mask;
                    },
                    "y" => {
                        y |= mask;
                    },
                    "z" => {
                        z |= mask;
                    },
                    _ => {},
                }
            }
        }
    }
    if DEBUG {
        println!("x: {x:046b}\ny: {y:046b}");
        println!("a: {z:046b}\ne: {:046b}", x+y);
        println!("{z}, {}", x+y);
    }
    let mut actual = vec![];
    while z > 0 {
        actual.push(z & 1);
        z = z >> 1;
    }
    return actual;
}

fn solve_p2(contents: String) -> String {
    let debug = DEBUG;
    let mut emu = Emulator::from_str(&contents).unwrap();
    let mut x = 0_u128;
    let mut y = 0_u128;
    let mut z = 0_u128;
    let keys = emu.mem.clone();
    let keys = keys.keys().collect::<Vec<&String>>();
    for name in keys {
        if &name[0..1] == "x" || &name[0..1] == "y" || &name[0..1] == "z" {
            let lshft = name[1..].parse::<usize>().unwrap();
            let mask = 1 << lshft;
            let val = emu.query(name).unwrap();
            if val {
                match &name[0..1] {
                    "x" => {
                        x |= mask;
                    },
                    "y" => {
                        y |= mask;
                    },
                    "z" => {
                        z |= mask;
                    },
                    _ => {},
                }
            }
        }
    }
    if debug {
        println!("{x:046b}\n{y:046b}");
        println!("{z:06b}\n{:06b}", x+y);
    }
    let mut actual = vec![];
    while z > 0 {
        actual.push(z & 1);
        z = z >> 1;
    }
    let mut expected = vec![];
    let mut e = x + y;
    while e > 0 {
        expected.push(e & 1);
        e = e >> 1;
    }
    if debug {
        println!("{expected:?}");
        println!("{actual:?}");
    }
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    emu.walk(&String::from("z03"), 0, 3);
    emu.walk(&String::from("z13"), 0, 3);
    // swap wss and wrm
    let binding = emu.mem.clone();
    let tmp  = binding.get(&String::from("wss")).unwrap();
    emu.mem.insert(String::from("wss"), emu.mem.get(&String::from("wrm")).unwrap().clone());
    emu.mem.insert(String::from("wrm"), tmp.clone());
    emu.walk(&String::from("z14"), 0, 3);
    emu.cache.clear();
    let actual = compute_output(&mut emu.clone());
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    emu.walk(&String::from("z29"), 0, 3);
    // XOR of xy29 -> bfq
    // OR of (AND xy28) (AND (XOR xy28) (OR z28 carry bit, XOR prev input))
    // OR ( XOR inputs ), ( AND prev_inputs )
    // z29 -> XOR ( XOR xy29), ( AND z28_inputs )
    emu.walk(&String::from("z28"), 0, 3);
    emu.walk(&String::from("z27"), 0, 3);
    // z29 -> XOR ( XOR xy29), ( AND bst, stp )
    // XOR ( bfq ), ( OR ( AND z28_inputs, ( AND xy28 ) ) )
    // OR ( AND bst, stp ) ( AND x28, y28 )
    // OR ( AND bst, stp ) ( rhg )
    // OR ( pdq ) ( rhg )
    // XOR ( bfq ), ( dcf )
    // gbs
    let binding = emu.mem.clone();
    let tmp  = binding.get(&String::from("z29")).unwrap();
    emu.mem.insert(String::from("z29"), emu.mem.get(&String::from("gbs")).unwrap().clone());
    emu.mem.insert(String::from("gbs"), tmp.clone());
    // emu.walk(&String::from("z29"), 0, 3);
    emu.cache.clear();
    let actual = compute_output(&mut emu.clone());
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    // now everything works so we need to start messing with the input??
    // lets try setting x to all 1's
    let mut x = 0;
    let mut y = 0;
    for i in 0..46 {
        let x_str = format!("x{i:02}");
        emu.mem.insert(x_str, Def::Val(true));
        let y_str = format!("y{i:02}");
        emu.mem.insert(y_str, Def::Val(true));
        x |= 1 << i;
        y |= 1 << i;
    }
    emu.cache.clear();
    let mut expected = vec![];
    let mut e = x + y;

    while e > 0 {
        expected.push(e & 1);
        e = e >> 1;
    }
    let actual = compute_output(&mut emu.clone());
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    emu.walk(&String::from("z07"), 0, 3);
    emu.walk(&String::from("z08"), 0, 3);
    // XOR ( XOR xy08 ), ( OR ( AND z07_inputs, ( AND xy07 ) ) )
    let binding = emu.mem.clone();
    let tmp  = binding.get(&String::from("z08")).unwrap();
    emu.mem.insert(String::from("z08"), emu.mem.get(&String::from("thm")).unwrap().clone());
    emu.mem.insert(String::from("thm"), tmp.clone());
    // emu.walk(&String::from("z29"), 0, 3);
    emu.cache.clear();
    let actual = compute_output(&mut emu.clone());
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    let mut x = 0_u128;
    let mut y = 0_u128;
    for i in 0..46 {
        let x_str = format!("x{i:02}");
        emu.mem.insert(x_str, Def::Val(if i % 2 == 0 { true } else { false }));
        let y_str = format!("y{i:02}");
        emu.mem.insert(y_str, Def::Val(if i % 2 != 0 { true } else { false }));
        if i % 2 == 0 {
            x |= 1 << i;
        }
        else {
            y |= 1 << i;
        }
    }
    if debug {
        println!("{x} {y}");
    }
    emu.cache.clear();
    let mut expected = vec![];
    let mut e = x + y;
    while e > 0 {
        if debug {
            println!("{:046b}", e);
        }
        expected.push(e & 1);
        e = e >> 1;
    }
    let actual = compute_output(&mut emu.clone());
    if debug {
        println!("{actual:?}");
        println!("{expected:?}");
    }
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    emu.walk(&String::from("z21"), 0, 3);
    emu.walk(&String::from("z22"), 0, 3);
    // XOR ( XOR xy22 ), ( OR ( ( AND z21_inputs ), ( AND xy21 ) ) )
    // XOR ( cdf ), ( OR ( ntb , ( qwd ) ) )
    // XOR ( cdf ), ( cmn )
    // XOR ( cdf ), ( cmn )
    // hwq
    let binding = emu.mem.clone();
    let tmp  = binding.get(&String::from("z22")).unwrap();
    emu.mem.insert(String::from("z22"), emu.mem.get(&String::from("hwq")).unwrap().clone());
    emu.mem.insert(String::from("hwq"), tmp.clone());
    // emu.walk(&String::from("z29"), 0, 3);
    emu.cache.clear();
    let actual = compute_output(&mut emu.clone());
    for i in 0..actual.len() {
        if expected[i] != actual[i] {
            if debug {
                println!("{i}");
            }
        }
    }
    let mut ans = vec!["wss", "wrm", "z22", "hwq", "z08", "thm", "z29", "gbs"];
    ans.sort();
    format!("{}", ans.join(","))
}

fn solve_p1(contents: String) -> i64 {
    let mut emu = Emulator::from_str(&contents).unwrap();
    let mut total = 0;
    let keys = emu.mem.clone();
    let keys = keys.keys().collect::<Vec<&String>>();
    for name in keys {
        if &name[0..1] == "z" {
            let lshft = name[1..].parse::<usize>().unwrap();
            let mask = 1 << lshft;
            let val = emu.query(name).unwrap();
            if val {
                total |= mask;
            }
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_24.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 4);
}

