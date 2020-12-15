use std::collections::VecDeque;
use itertools::Itertools;

use crate::*;

pub struct Challenge15;

impl Challenge for Challenge15 {
    fn filename(&self) -> &'static str {
        "15a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut numbers: VecDeque<u64> = VecDeque::new();
        numbers.extend(input.split(',').map(|x| x.parse::<u64>().unwrap()));
        println!("{:?}", numbers);
        let count = numbers.len();
        
        for _ in count..2020 {
            let last = numbers.back().unwrap();
            if let Some((dist, _)) = numbers.iter().rev().skip(1).find_position(|&x| x == last) {
                numbers.push_back(dist as u64 + 1);
            } else {
                numbers.push_back(0);
            }
            
            println!("pushed {}", numbers.back().unwrap());
        }
        
        Ok(numbers.back().unwrap().to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}