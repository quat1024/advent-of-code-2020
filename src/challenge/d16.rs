use std::ops::RangeInclusive;
use itertools::Itertools;

use crate::*;

pub struct Challenge16;

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

impl Input {
    fn from_string(input: &str) -> Input {
        let mut blocks = input.split("\n\n"); //sorry to CRLF users
        
        Input {
            rules: blocks.next().unwrap().lines().map(Rule::from_string).collect(),
            my_ticket: blocks.next().unwrap().lines().nth(1).map(Ticket::from_string).unwrap(),
            nearby_tickets: blocks.next().unwrap().lines().skip(1).map(Ticket::from_string).collect()
        }
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: (RangeInclusive<u16>, RangeInclusive<u16>)
}

impl Rule {
    fn from_string(input: &str) -> Rule {
        let (name, rest) = input.split(':').collect_tuple().unwrap();
        Rule {
            name: name.to_owned(),
            ranges: rest.trim().split(" or ").map(|unparsed_range| -> RangeInclusive<u16> {
                let (start, end) = unparsed_range.split('-').map(|s| s.parse::<u16>().unwrap()).collect_tuple().unwrap();
                start ..= end
            }).collect_tuple().unwrap()
        }
    }
    
    fn contains(&self, n: u16) -> bool {
        return self.ranges.0.contains(&n) || self.ranges.1.contains(&n);
    }
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<u16>
}

impl Ticket {
    fn from_string(input: &str) -> Ticket {
        Ticket {
            numbers: input.split(',').map(|s| s.parse::<u16>().unwrap()).collect()
        }
    }
}

impl Challenge for Challenge16 {
    fn filename(&self) -> &'static str {
        "16a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let input = Input::from_string(&input);
        
        let error_rate: u16 = input.nearby_tickets.iter().flat_map(|t| t.numbers.iter()).filter(|&&n| !input.rules.iter().any(|r| r.contains(n))).sum();
        
        Ok(error_rate.to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}