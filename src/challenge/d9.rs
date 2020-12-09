use crate::*;

use itertools::Itertools;
use std::collections::HashSet;

pub struct Challenge9;

impl Challenge for Challenge9 {
    fn filename(&self) -> &'static str {
        "9a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        for window in input
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .array_windows::<26>()
        {
            //Probably an elegant iterator solution but im just bangin out code here
            let mut sums = HashSet::new();

            for i in 0..25 {
                for j in (i + 1)..25 {
                    sums.insert(window[i] + window[j]);
                }
            }

            if !sums.contains(&window[25]) {
                return Ok(window[25].to_string());
            }
        }

        unimplemented!()
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let target = 248131121; //from part a
        let input = input
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let range_bounds = find_range_bounds(&input, target);
        
        let smallest = input[range_bounds.0..=range_bounds.1].iter().min().unwrap();
        let largest = input[range_bounds.0..=range_bounds.1].iter().max().unwrap();
        return Ok((smallest + largest).to_string());
        
        //unimplemented!()
    }
}

fn find_range_bounds(input: &Vec<usize>, target: usize) -> (usize, usize) {
    let mut start = 0;
    loop {
        let mut sum = 0;
        for i in start..999999 {
            sum += input[i];
            if sum == target {
                return (start, i);
            } else if sum >= target {
                break;
            }
        }
        start += 1;
    }
    
    unimplemented!()
}
