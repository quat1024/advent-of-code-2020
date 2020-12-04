use crate::*;
use itertools::Itertools;

pub struct Challenge4;

impl Challenge for Challenge4 {
    fn filename(&self) -> &'static str {
        "4a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        use std::collections::HashSet;

        Ok(input
            .lines()
            .group_by(|line| line.trim().is_empty()) //itertools
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
            .count()
            .to_string())
    }

    //Wrong: 122
    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        //Oh dang now i actually have to parse stuff!
        use regex::Regex;
        use std::collections::HashSet;

        //let hgt_regex = Regex::new(r"(?P<num>[0-9]+)(?P<unit>cm|in)").unwrap();
        let hcl_regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
        let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();

        Ok(input
            .lines()
            .group_by(|line| line.trim().is_empty()) //itertools
            .into_iter()
            .filter_map(|(blank, group)| -> Option<HashSet<&str>> {
                if blank {
                    return None;
                }

                let mut keys: HashSet<&str> = HashSet::new();

                group
                    .flat_map(|s| s.split_whitespace())
                    .filter_map(|s: &str| -> Option<&str> {
                        let mut parts = s.split(':');
                        let key = parts.next()?;
                        let value = parts.next()?;

                        let keep = match key {
                            "byr" => {
                                let number = value.parse::<usize>().ok()?;
                                number >= 1920 && number <= 2002
                            }
                            "iyr" => {
                                let number = value.parse::<usize>().ok()?;
                                number >= 2010 && number <= 2020
                            }
                            "eyr" => {
                                let number = value.parse::<usize>().ok()?;
                                number >= 2020 && number <= 2030
                            }
                            "hgt" => {
                                //This sucks lol
                                let mut ret = false;
                                if let Some(s) = value.strip_suffix("cm") {
                                    let number = s.parse::<usize>().ok()?;
                                    ret = number >= 150 && number <= 193
                                }

                                if let Some(s) = value.strip_suffix("in") {
                                    let number = s.parse::<usize>().ok()?;
                                    ret = number >= 59 && number <= 76
                                }

                                ret
                            }
                            "hcl" => hcl_regex.is_match(value),
                            "ecl" => match value {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                                _ => false,
                            },
                            "pid" => pid_regex.is_match(value),
                            "cid" => true,
                            _ => false,
                        };

                        if keep {
                            //println!("{}\tVALID", s);
                            Some(key)
                        } else {
                            //println!("{}\tINVALID", s);
                            None
                        }
                    })
                    .for_each(|key| -> () {
                        keys.insert(key);
                    });

                Some(keys)
            })
            .filter(|set| set.len() == 8 || set.len() == 7 && !set.contains("cid"))
            .count()
            .to_string())
    }
}
