
use std::str::FromStr;
use std::collections::HashSet;

pub fn solve() {
    let contents = std::fs::read_to_string("input_7.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

fn solve_p2(contents: String) -> i64 {
    let eqs = parse_input(contents);
    eqs.iter()
        .filter(|eq| {
            eq.has_valid_solution_dp_1(2)
        })
        .map(|eq| {
            eq.result
        }).sum()

}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_7.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 11387);
}

#[derive(Clone)]
enum Operator {
    Add,
    Mul,
    Cat,
}

#[derive(Debug)]
struct Equation {
    operands: Vec<i64>,
    result: i64,
}

impl FromStr for Equation {
    type Err = ();
    fn from_str(raw: &str) -> Result<Equation, ()> {
        let split: Vec<&str> = raw.split(":").collect();
        let result = split[0].parse::<i64>().map_err(|_e| {
            println!("ERROR: [{}], [{}]", split[0], raw);
            return Err::<Equation, ()>(());
        }).unwrap();
        let operands: Vec<i64> = split[1].split(" ")
            .filter(|s| { s.len() > 0 })
            .map(|c| {
                c.parse::<i64>().expect("INVALID OPERAND")
            })
            .collect();
        Ok(Equation {
            operands,
            result,
        })
    }
}


#[allow(dead_code)]
impl Equation {

    // valid solution if cat had higher precedence than other operators
    // yet another cool but useless implementation
    fn has_valid_solution_cat(&self) -> bool {
        let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![]]; self.operands.len()];
        dp[0] = vec![vec![self.operands[0]]];
        let mut i = 1;
        while i < dp.len() {
            let mut curr: Vec<Vec<i64>> = vec![];
            let mut j = 0;
            while j < dp[i-1].len() {
                // concat
                let mut cat = dp[i-1][j].clone();
                let idx = cat.len()-1;
                cat[idx] = format!("{}{}", cat[idx], self.operands[i]).parse::<i64>().unwrap();
                curr.push(cat);
                // don't concat
                let mut ncat = dp[i-1][j].clone();
                ncat.push(self.operands[i]);
                curr.push(ncat);
                j += 1;
            }
            dp[i] = curr;
            i += 1;
        }
        // println!("{:?}", dp[dp.len()-1]);
        dp[dp.len()-1].iter().any(|operands| {
            let eq = Equation {
                operands: operands.to_vec(),
                result: self.result,
            };
            let valid = eq.has_valid_solution_dp_1(1);
            // println!("{eq:?} {valid}");
            valid

        })
    }

    // ~15 seconds for part 2 down from ~2 minutes w/ the other dp solution
    // trimming down the number of computations by only computing the
    // possible next steps from unique values in the previous iteration
    fn has_valid_solution_dp_1(&self, part: u8) -> bool {
        let ops = match part {
            1 => {
                vec![Operator::Add, Operator::Mul]
            },
            2 => {
                vec![Operator::Add, Operator::Mul, Operator::Cat]
            },
            _ => panic!("There are only parts 1 & 2"),
        };
        // store each iteration results in a hashset & throw out previous
        // iteration when the next has been computed
        // this is ok because we only care about possible results,
        // not how we got here
        let mut current = HashSet::<i64>::new();
        current.insert(self.operands[0]);
        let mut i = 1;
        while i < self.operands.len() {
            let mut next = HashSet::<i64>::new();
            for dpj in current {
                for op in &ops {
                    let val = match op {
                        Operator::Add => {
                            dpj + self.operands[i]
                        },
                        Operator::Mul => {
                            dpj * self.operands[i]
                        },
                        Operator::Cat => {
                            format!("{}{}", dpj, self.operands[i]).parse::<i64>().unwrap()
                        },
                    };
                    next.insert(val);
                }
            }
            current = next;
            i += 1;
        }
        current.iter().any(|r| { *r == self.result })
    }

    // this solution is significantly slower for part 2 than '..'_dp_1()
    // because it computes the entire dp and doesn't deduplicate any results
    fn has_valid_solution_dp(&self, part: u8) -> bool {
        let ops = match part {
            1 => {
                vec![Operator::Add, Operator::Mul]
            },
            2 => {
                vec![Operator::Add, Operator::Mul, Operator::Cat]
            },
            _ => panic!("There are only parts 1 & 2"),
        };
        // start with just the first element
        let mut dp: Vec<Vec<i64>> = vec![vec![]; self.operands.len()];
        dp[0] = vec![self.operands[0]];
        // in each iteration, compute all possible results given the operators
        // from each of the previous solutions
        let mut i = 1;
        while i < dp.len() {
            let mut curr = vec![];
            let mut j = 0;
            while j < dp[i-1].len() {
                let dpj = &dp[i-1][j];
                for op in &ops {
                    let val = match op {
                        Operator::Add => {
                            dpj + self.operands[i]
                        },
                        Operator::Mul => {
                            dpj * self.operands[i]
                        },
                        Operator::Cat => {
                            format!("{}{}", dpj, self.operands[i]).parse::<i64>().unwrap()
                        },
                    };
                    curr.push(val);
                }
                j += 1;
                dp[i] = curr.clone();
            }
            i += 1;
        }
        // by the end we have all possible solutions to this equation
        dp[dp.len()-1].iter().any(|r| { *r == self.result })
    }

    // the naive approach
    fn has_valid_solution(&self, part: u8) -> bool {
        // for every possible combination of operands.len()-1 operators
        let mut operators = vec![Operator::Add; self.operands.len()-1];
        let search_space: u128 = match part {
            1 => 2_u128.pow((operators.len() as u128).try_into().unwrap()),
            _ => panic!("This approach only works for part 1"),
        };
        // use some bit magic to get all possible combinations
        let mut bts: u128 = 0;
        while bts < search_space {
            let mut i = 0;
            while i < operators.len() {
                match part  {
                    1 => {
                        // count base 2
                        operators[i] = match (bts>>i)&1 {
                            0 => Operator::Add,
                            1 => Operator::Mul,
                            _ => unreachable!("This shouldn't happen i think?"),
                        };
                        i += 1;
                    },
                    _ => panic!("There are only parts 1 & 2"),
                }
            }
            println!("");
            if self.eval(&operators) {
                return true;
            }
            bts += 1;
        }
        false
    }

    fn eval(&self, ops: &Vec<Operator>) -> bool {
        let mut total = self.operands[0];
        for i in 1..self.operands.len() {
            match ops[i-1] {
                Operator::Cat => {
                    total = format!("{}{}", total, self.operands[i])
                        .parse::<i64>().unwrap();
                },
                Operator::Mul => {
                    total *= self.operands[i];
                },
                Operator::Add => {
                    total += self.operands[i];
                }
            }
        }
        total == self.result
    }

    // eval with cat having higher precedence than add and mul
    // definitely didn't write this because I forgot everything gets evaluated
    // from left to right
    fn eval_cat_prec(&self, ops: &Vec<Operator>) -> bool {
        let mut operand_idx = 0;
        let mut stack: Vec<i64> = vec![self.operands[operand_idx]];
        while operand_idx < self.operands.len()-1 {
            operand_idx += 1;
            match ops[operand_idx-1] {
                Operator::Cat => {
                    let left = stack.remove(0);
                    let right = self.operands[operand_idx];
                    stack.insert(0, (left * 10_i64.pow(format!("{right}").len() as u32)) + right);
                },
                Operator::Add => {
                    stack[0] += self.operands[operand_idx];
                },
                Operator::Mul => {
                    stack[0] *= self.operands[operand_idx];
                },
            }
        }
        stack[0] == self.result
    }
}

fn parse_input(contents: String) -> Vec<Equation> {
    contents.lines().map(|line| {
        Equation::from_str(line).unwrap()
    }).collect::<Vec<Equation>>()
}

fn solve_p1(contents: String) -> i64 {
    let eqs = parse_input(contents);
    let mut total = 0;
    for eq in eqs {
        if eq.has_valid_solution_dp_1(1) {
            total += eq.result;
        }
    }
    total
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_7.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == 3749);
}
