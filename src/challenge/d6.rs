use crate::*;
use itertools::Itertools;

pub struct Challenge6;

impl Challenge for Challenge6 {
    fn filename(&self) -> &'static str {
        "6a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        Ok(input.lines().group_by(|line| line.trim().is_empty()).into_iter().filter_map(|(blank, group)| -> Option<usize> {
            if blank {
                return None;
            }
            
            Some(group.collect::<String>().chars().unique().count())
        }).sum::<usize>().to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        Ok(input.lines().group_by(|line| line.trim().is_empty()).into_iter().filter_map(|(blank, group)| -> Option<usize> {
            if blank {
                return None;
            }
            
            let people: Vec<&str> = group.collect();
            let mut count = 0;
            
            for c in 'a'..='z' {
               if people.iter().all(|s| s.contains(c)) {
                   count += 1;
               } 
            }
            
            Some(count)
        }).sum::<usize>().to_string())
    }
}