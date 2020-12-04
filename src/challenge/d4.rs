use crate::*;
use itertools::Itertools;

pub struct Challenge4;

impl Challenge for Challenge4 {
    fn filename(&self) -> &'static str {
        "4a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        use std::collections::HashSet;

        let count = input
            .lines()
            .group_by(|line| line.trim().is_empty())
            .into_iter()
            .filter_map(|(blank, group)| -> Option<HashSet<&str>> {
                if blank {
                    return None;
                }

                let mut keys: HashSet<&str> = HashSet::new();

                group
                    .flat_map(|s| s.split_whitespace())
                    .filter_map(|s| s.split(':').next())
                    .for_each(|key| -> () {
                        keys.insert(key);
                    });

                Some(keys)
            })
            .filter(|set| set.len() == 8 || set.len() == 7 && !set.contains("cid"))
            .count();
            
        Ok(count.to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}
