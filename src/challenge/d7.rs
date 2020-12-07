use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::*;
use regex::Regex;

pub struct Challenge7;

lazy_static! {
    static ref BAG_NAME_REGEX: Regex = Regex::new(r"[a-z]+ [a-z]+").unwrap();
}

struct Rule {
    name: String,
    contents: Vec<ContentsRule>,
}

struct ContentsRule {
    qty: usize,
    color: String,
}

impl FromStr for Rule {
    type Err = ChallengeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //As usual TODO for the error handling being absolute garbo
        let mut spaced = s.split(' ');

        //The original intention was to grab a slice into the str that contains just the bag name
        //But I dunno how lol
        //Ran into lifetime issues as well later on
        let name = spaced.next().unwrap().to_string() + " " + spaced.next().unwrap();
        assert_eq!(spaced.next(), "bags".into());
        assert_eq!(spaced.next(), "contain".into());

        let mut contents: Vec<ContentsRule> = Vec::new();

        while let Some(s) = spaced.next() {
            if s == "no" {
                assert_eq!(spaced.next(), Some("other".into()));
                assert_eq!(spaced.next(), Some("bags.".into()));
                assert_eq!(spaced.next(), None);
                break;
            }

            if let Ok(qty) = s.parse::<usize>() {
                let color = spaced.next().unwrap().to_string() + " " + spaced.next().unwrap();
                contents.push(ContentsRule { qty, color });

                let _bags = spaced.next(); //"bag," "bags," "bag.", or "bags."
            }
        }

        Ok(Rule { name, contents })
    }
}

impl Challenge for Challenge7 {
    fn filename(&self) -> &'static str {
        "7a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let rules: Vec<Rule> = input.lines().map(|s| Rule::from_str(&s).unwrap()).collect();

        let reverse_lookup = |name: &str| -> Vec<&str> {
            let mut containers: Vec<&str> = Vec::new();

            for rule in rules.iter() {
                if rule.contents.iter().any(|c| c.color == name) {
                    containers.push(&rule.name);
                }
            }

            containers
        };

        let mut to_lookup: HashSet<&str> = HashSet::new();
        let mut found: HashSet<&str> = HashSet::new();

        to_lookup.insert("shiny gold".into());

        while !to_lookup.is_empty() {
            let to_lookup_clone = to_lookup.clone();
            to_lookup.clear();

            for name in to_lookup_clone {
                for found_name in reverse_lookup(name).iter() {
                    if !found.contains(found_name) {
                        found.insert(found_name);
                        to_lookup.insert(found_name);
                    }
                }
            }
        }

        Ok(found.len().to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let rules: HashMap<String, _> = input
            .lines()
            .map(|s| Rule::from_str(&s).unwrap())
            .map(|r| (r.name.clone(), r)) //WHEN IN DOUBT JUST CLONE EVERYTHING
            .collect();

        let mut cache: HashMap<String, usize> = HashMap::new();

        Ok(count_bags_inside(&rules, &mut cache, "shiny gold".into()).to_string())
    }
}

fn count_bags_inside(
    rules: &HashMap<String, Rule>,
    cache: &mut HashMap<String, usize>,
    bag: String,
) -> usize {
    if cache.contains_key(&bag) {
        return *cache.get(&bag).unwrap();
    }

    println!("looking up {}", bag);

    let entry = rules.get(&bag);
    match entry {
        None => {
            println!("unknown bag {}", bag);
            return 0;
        }
        Some(rule) => {
            let mut sum = 0;
            for contents_rule in rule.contents.iter() {
                sum += contents_rule.qty
                    * (1 + count_bags_inside(rules, cache, contents_rule.color.clone()))
            }
            println!("{} can hold {}", bag, sum);
            cache.insert(bag, sum);
            sum
        }
    }
}
