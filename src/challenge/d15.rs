use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use crate::*;

pub struct Challenge15;

impl Challenge for Challenge15 {
    fn filename(&self) -> &'static str {
        "15a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut numbers: VecDeque<u64> = VecDeque::new();
        numbers.extend(input.split(',').map(|x| x.parse::<u64>().unwrap()));
        let count = numbers.len();

        for _ in count..2020 {
            let last = numbers.back().unwrap();
            if let Some((dist, _)) = numbers.iter().rev().skip(1).find_position(|&x| x == last) {
                numbers.push_back(dist as u64 + 1);
            } else {
                numbers.push_back(0);
            }
        }

        Ok(numbers.back().unwrap().to_string())
    }

    //wrong: 17
    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut numbers: VecDeque<u64> = VecDeque::new();
        let mut seen_numbers: HashMap<u64, usize> = HashMap::new();

        numbers.extend(input.split(',').map(|x| x.parse::<u64>().unwrap()));

        for i in 0..(numbers.len() - 1) {
            seen_numbers.insert(numbers[i], i + 1);
        }

        let count = numbers.len();

        let mut last_pushed = *numbers.back().unwrap();

        for step in count..30_000_000 {
            if !seen_numbers.contains_key(&last_pushed) {
                seen_numbers.insert(last_pushed, step);
                last_pushed = 0;
            } else {
                let when_seen = seen_numbers.get(&last_pushed).unwrap().clone();
                seen_numbers.insert(last_pushed, step);
                last_pushed = (step - when_seen) as u64;
            }

            if step % 1_000_000 == 0 {
                println!("on step {}", step);
            }
        }

        Ok(last_pushed.to_string())
    }
}
