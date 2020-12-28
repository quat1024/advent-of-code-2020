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
        self.ranges.0.contains(&n) || self.ranges.1.contains(&n)
    }
    
    fn is_departure_info(&self) -> bool {
        self.name.starts_with("departure")
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

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut input = Input::from_string(&input);
        
        //Discard invalid tickets
        let rules = &input.rules; //borrow checker ;)
        input.nearby_tickets.retain(|ticket| !ticket.numbers.iter().any(|&n| !rules.iter().any(|rule| rule.contains(n))));
        
        //(double-check my work)
        'next_ticket:
        for ticket in input.nearby_tickets.iter() {
            for &n in ticket.numbers.iter() {
                if rules.iter().any(|rule| rule.contains(n)) {
                    continue 'next_ticket;
                }
                panic!("this number: {} is not valid for any rules", n)
            }
        }
        
        let ticket_count = input.nearby_tickets.len();
        println!("leftover tickets: {}", ticket_count);
        
        let field_count = input.my_ticket.numbers.len();
        if input.nearby_tickets.iter().any(|t| t.numbers.len() != field_count) {
            panic!("not all tickets are the same length");
        }
        
        let rules_count = input.rules.len();
        
        for rule in rules.iter() {
            for i in 0..field_count {
                if input.nearby_tickets.iter().all(|ticket| rule.contains(ticket.numbers[i])) {
                    println!("rule {} can be applied to field {}", rule.name, i);
                }
            }
        }
        
        //todo
        
        Err(ChallengeErr::NotYetImplemented())
    }
}