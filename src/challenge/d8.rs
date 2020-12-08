use crate::*;

use std::collections::HashSet;

pub struct Challenge8;

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Opcode {
    fn flip(&mut self) {
        *self = match self {
            Opcode::Acc(mut a) => Opcode::Acc(a),
            Opcode::Nop(mut a) => Opcode::Jmp(a),
            Opcode::Jmp(mut a) => Opcode::Nop(a),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Status {
    Running,
    Halted,
}

struct Computer {
    opcodes: Vec<Opcode>,
    visited: HashSet<usize>, //This could be a lot better, like a bitset or smtn
    acc: i32,
    pc: i32,
    status: Status,
}

impl Computer {
    fn new(input: String) -> Computer {
        let opcodes = input
            .lines()
            .map(|l| -> Opcode {
                let mut unparsed_opcode = l.split(' ');
                let a = unparsed_opcode.next().unwrap();
                let value = unparsed_opcode.next().unwrap().parse::<i32>().unwrap();
                match a {
                    "nop" => Opcode::Nop(value),
                    "acc" => Opcode::Acc(value),
                    "jmp" => Opcode::Jmp(value),
                    _ => panic!(),
                }
            })
            .collect();

        Computer {
            opcodes,
            visited: HashSet::new(),
            acc: 0,
            pc: 0,
            status: Status::Running,
        }
    }

    fn step(&mut self) {
        if self.status == Status::Halted {
            return;
        }

        self.visited.insert(self.pc as usize);

        if self.pc == self.opcodes.len() as i32 {
            self.status = Status::Halted;
        } else {
            match self.opcodes.get(self.pc as usize) {
                Some(Opcode::Nop(_)) => {
                    self.pc += 1;
                }
                Some(Opcode::Acc(val)) => {
                    self.acc += val;
                    self.pc += 1;
                }
                Some(Opcode::Jmp(val)) => {
                    self.pc += val;
                }
                None => panic!(),
            }
        }
    }

    fn reset(&mut self) {
        self.acc = 0;
        self.pc = 0;
        self.visited.clear();
        self.status = Status::Running;
    }

    fn has_been_here(&self) -> bool {
        self.visited.contains(&(self.pc as usize))
    }

    fn is_halted(&self) -> bool {
        self.status == Status::Halted
    }
}

impl Challenge for Challenge8 {
    fn filename(&self) -> &'static str {
        "8a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut computer = Computer::new(input);

        while !computer.has_been_here() {
            computer.step()
        }

        Ok(computer.acc.to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut computer = Computer::new(input);
        let insns_clone = computer.opcodes.clone();

        let mut idx = 0;
        loop {
            while let Some(Opcode::Acc(_)) = insns_clone.get(idx) {
                idx += 1;
            }
            println!("trying flipping index {}", idx);

            if insns_clone.get(idx).is_none() {
                return Err(ChallengeErr::Unspecified("No solution".into()));
            }

            let mut new_insns = insns_clone.clone();
            new_insns[idx].flip();
            computer.reset();
            computer.opcodes = new_insns;

            //let mut tries = 0;
            while !computer.has_been_here() && !computer.is_halted() {
                computer.step();
                //tries += 1;
            }

            if computer.is_halted() {
                return Ok(computer.acc.to_string());
            }

            idx += 1;
        }
    }
}
