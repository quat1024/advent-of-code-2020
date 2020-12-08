use crate::*;

use std::collections::HashSet;

pub struct Challenge8;

#[derive(Debug)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

struct Computer {
    opcodes: Vec<Opcode>,
    visited: HashSet<usize>, //This could be a lot better, like a bitset or smtn
    acc: i32,
    pc: i32
}

impl Computer {
    fn new(input: String) -> Computer {
        let opcodes = input.lines().map(|l| -> Opcode {
            let mut unparsed_opcode = l.split(' ');
            let a = unparsed_opcode.next().unwrap();
            let value = unparsed_opcode.next().unwrap().parse::<i32>().unwrap();
            match a {
                "nop" => Opcode::Nop(value),
                "acc" => Opcode::Acc(value),
                "jmp" => Opcode::Jmp(value),
                _ => panic!()
            }
        }).collect();
        
        let visited = HashSet::new();
        let acc = 0;
        let pc = 0;
        Computer {opcodes, visited, acc, pc}
    }
    
    fn step(&mut self) {
        self.visited.insert(self.pc as usize);
        
        match self.opcodes.get(self.pc as usize) {
            Some(Opcode::Nop(_)) => {
                self.pc += 1;
            },
            Some(Opcode::Acc(val)) => {
                self.acc += val;
                self.pc += 1;
            },
            Some(Opcode::Jmp(val)) => {
                self.pc += val;
            }
            None => panic!()
        }
    }
    
    fn has_been_here(&self) -> bool {
        self.visited.contains(&(self.pc as usize))
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

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}