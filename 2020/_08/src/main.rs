use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::newline;
use nom::combinator::{eof, map};
use nom::multi::many_till;
use nom::sequence::{pair, terminated};
use nom::IResult;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::Read;
use util::Result;

#[derive(Copy, Clone, Debug)]
enum Instr {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

struct Cpu {
    code: Vec<Instr>,
    acc: i32,
    pc: i32,
}

impl Cpu {
    fn new(code: Vec<Instr>) -> Cpu {
        Cpu {
            code,
            acc: 0,
            pc: 0,
        }
    }

    /// Returns the program counter after stepping
    fn step(&mut self) -> i32 {
        match self.code[self.pc as usize] {
            Instr::Acc(val) => {
                self.acc += val;
                self.pc += 1;
            }
            Instr::Nop(_) => {
                self.pc += 1;
            }
            Instr::Jmp(val) => self.pc += val,
        }
        self.pc
    }
}

fn main() -> Result<()> {
    let mut buf = String::new();
    let mut file = std::fs::File::open("_08/input.txt")?;
    file.read_to_string(&mut buf)?;
    let code = parse(&buf)?;
    let cpu = Cpu::new(code.clone());
    let infinite_loop = find_infinite_loop(cpu);
    let corrupt_fix = find_corrupt_instr(code);
    println!("{}", infinite_loop);
    println!("{}", corrupt_fix);
    Ok(())
}

fn find_corrupt_instr(orig_instructions: Vec<Instr>) -> i32 {
    let success_pc = i32::try_from(orig_instructions.len()).unwrap();
    let mut instr_to_swap = 0;
    loop {
        let mut copy = orig_instructions.clone();
        match copy[instr_to_swap] {
            Instr::Jmp(val) => copy[instr_to_swap] = Instr::Nop(val),
            Instr::Nop(val) => copy[instr_to_swap] = Instr::Jmp(val),
            _ => {
                instr_to_swap += 1;
                continue;
            }
        }
        let mut cpu = Cpu::new(copy);
        for _ in 0..1000 {
            if cpu.step() == success_pc {
                break;
            }
        }

        if cpu.pc == success_pc {
            break cpu.acc;
        } else if instr_to_swap == orig_instructions.len() - 1 {
            panic!("No solution fond");
        }
        instr_to_swap += 1;
    }
}

fn find_infinite_loop(mut cpu: Cpu) -> i32 {
    let mut executed_instructions = HashSet::<i32>::default();
    loop {
        let acc = cpu.acc;
        let pc = cpu.step();
        if executed_instructions.contains(&pc) {
            break acc;
        } else {
            executed_instructions.insert(pc);
        }
    }
}

fn parse(input: &str) -> std::result::Result<Vec<Instr>, &'static str> {
    fn parse_instr(input: &str) -> IResult<&str, Instr> {
        terminated(
            map(
                pair(
                    terminated(alt((tag("nop"), tag("acc"), tag("jmp"))), tag(" ")),
                    // map(
                    map(
                        pair(
                            map(alt((tag("+"), tag("-"))), |sign| match sign {
                                "+" => 1,
                                "-" => -1,
                                _ => unimplemented!(),
                            }),
                            map(take_until("\n"), |val: &str| val.parse::<i32>().unwrap()),
                        ),
                        |(sign, val)| sign * val,
                    ),
                ),
                |(instr, val)| match instr {
                    "nop" => Instr::Nop(val),
                    "acc" => Instr::Acc(val),
                    "jmp" => Instr::Jmp(val),
                    _ => unimplemented!(),
                },
            ),
            newline,
        )(input)
    }

    let (_, (instructions, _)) =
        many_till(parse_instr, eof)(input).map_err(|_| "Failed to parse")?;
    Ok(instructions)
}
