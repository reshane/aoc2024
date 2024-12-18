
pub fn solve() {
    let contents = std::fs::read_to_string("input_17.txt").expect("WHERE IS THE FILE");
    println!("part 1: {}", solve_p1(contents.clone()));
    println!("part 2: {}", solve_p2(contents));
}

use std::str::FromStr;
use std::fmt::Display;
use std::string::ToString;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn to_raw(&self) -> u64 {
        match self {
            Opcode::Adv => 0,
            Opcode::Bxl => 1,
            Opcode::Bst => 2,
            Opcode::Jnz => 3,
            Opcode::Bxc => 4,
            Opcode::Out => 5,
            Opcode::Bdv => 6,
            Opcode::Cdv => 7,
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_raw())
    }
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        if let Ok(opcode) = raw.parse::<usize>() {
            return match opcode {
                0 => Ok(Opcode::Adv),
                1 => Ok(Opcode::Bxl),
                2 => Ok(Opcode::Bst),
                3 => Ok(Opcode::Jnz),
                4 => Ok(Opcode::Bxc),
                5 => Ok(Opcode::Out),
                6 => Ok(Opcode::Bdv),
                7 => Ok(Opcode::Cdv),
                _ => Err(()),
            };
        }
        Err(())
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: u64,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(raw: &str) -> Result<Instruction, ()> {
        if let Some((opcode_raw, operand_raw)) = raw.split_once(",") {
            if let (Ok(opcode), Ok(operand)) = (Opcode::from_str(opcode_raw), operand_raw.parse::<u64>()) {
                return Ok(Instruction {
                    opcode,
                    operand,
                });
            }
            return Err(());
        }
        Err(())
    }
}

#[derive(Debug)]
struct Program {
    ins: Vec<Instruction>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.ins
            .iter()
            .map(|ins| {
                format!("{},{}", ins.opcode, ins.operand)
            })
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl FromStr for Program {
    type Err = ();
    fn from_str(raw: &str) -> Result<Program, ()> {
        let raw = &raw[raw.find(":").unwrap()+1..];
        let ins = raw.split(",")
            .map(|o| { o.trim() })
            .collect::<Vec<&str>>()
            .chunks(2)
            .filter_map(|pair| {
                Instruction::from_str(&pair.join(",")).ok()
            })
            .collect::<Vec<Instruction>>();
        Ok(Program {
            ins
        })
    }
}

#[derive(Default)]
struct Machine {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
}

impl Machine {

    fn reset(&mut self) {
        self.ip = 0;
        self.a = 0;
        self.b = 0;
        self.c = 0;
    }

    fn run_program(&mut self, program: &Program) -> String {
        let mut output = String::default();
        while self.ip < program.ins.len() {
            let current = program.ins[self.ip].clone();
            let mut jnz = false;
            match current.opcode {
                Opcode::Adv => {
                    self.a = self.a / (1 << self.resolve_combo(current.operand));
                },
                Opcode::Bdv => {
                    self.b = self.a / (1 << self.resolve_combo(current.operand));
                },
                Opcode::Cdv => {
                    self.c = self.a / (1 << self.resolve_combo(current.operand));
                },
                Opcode::Out => {
                    if !output.is_empty() {
                        output.push_str(",");
                    }
                    output.push_str(format!("{}", self.resolve_combo(current.operand) % 8).as_str());
                },
                Opcode::Jnz => {
                    if self.a != 0 {
                        self.ip = current.operand.try_into()
                            .expect(format!("Invalid Jnz instruction {current:?}").as_str());
                        jnz = true;
                    }
                },
                Opcode::Bxc => {
                    self.b = self.b ^ self.c;
                },
                Opcode::Bxl => {
                    self.b = self.b ^ current.operand;
                },
                Opcode::Bst => {
                    self.b = self.resolve_combo(current.operand) % 8;
                },
            }
            if !jnz {
                self.ip += 1;
            }
        }
        output
    }

    fn resolve_combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved combo operand 7: invalid program"),
            _ => panic!("unrecognized combo operand {}", operand),
        }
    }
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(raw: &str) -> Result<Machine, ()> {
        let regs = raw.lines()
            .map(|line| {
                line[line.find(":").unwrap()+2..]
                    .parse::<u64>().unwrap()
            })
            .collect::<Vec<u64>>();
        if regs.len() != 3 {
            return Err(());
        }
        Ok(Machine {
            ip: 0,
            a: regs[0],
            b: regs[1],
            c: regs[2],
        })
    }
}

fn parse_input(contents: String) -> (Machine, Program) {
    if let Some((raw_machine, raw_program)) = contents.split_once("\n\n") {
        return (
            Machine::from_str(raw_machine).expect("Invalid machine"),
            Program::from_str(raw_program).expect("Invalid program")
        );

    }
    unreachable!("Could not separate machine from program");
}

fn solve_p1(contents: String) -> String {
    let (mut machine, program) = parse_input(contents);
    machine.run_program(&program)
}

#[test]
fn test_sample_1() {
    let contents = std::fs::read_to_string("sample_17.txt").expect("WHERE IS THE FILE");
    let result = solve_p1(contents);
    println!("{result}");
    assert!(result == String::from("4,6,3,5,6,3,5,2,1,0"));
}


fn solve_p2(contents: String) -> u64 {
    let (_, program) = parse_input(contents);
    let mut machine = Machine::default();
    let program_string = program.to_string();

    // least significant 3 bits
    // because every time we output, we % 8
    // also this won't work for the first sample case
    // because it is dividing everything by 2
    // so everything is getting shifted by 2 bits
    // rather than 3

    let mut queue = VecDeque::<u64>::new();
    for i in 0..8 {
        queue.push_back(i);
    }

    while let Some(i) = queue.pop_front() {
        'j: for j in 0..8 {
            let i = (i << 3) + j;

            machine.a = i;
            let output = machine.run_program(&program);

            if output.len() > program_string.len() {
                machine.reset();
                continue 'j;
            }
            let idx = program_string.len() - output.len();
            if &program_string[idx..] == &output[0..] {
                if idx == 0 {
                    return i;
                } else {
                    queue.push_back(i);
                }
            }
            machine.reset();
        }
    }
    unreachable!("There is no output for the given input")
}

#[test]
fn test_sample_2() {
    let contents = std::fs::read_to_string("sample_17b.txt").expect("WHERE IS THE FILE");
    let result = solve_p2(contents);
    println!("{result}");
    assert!(result == 117440);
}
