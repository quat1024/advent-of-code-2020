use crate::*;

pub struct Challenge13;

impl Challenge for Challenge13 {
    fn filename(&self) -> &'static str {
        "13a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut lines = input.lines();
        let now: usize = lines.next().unwrap().parse().unwrap();
        let buses: Vec<usize> = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|f| f.parse().ok())
            .collect();

        //There's probably a nice way to do this but brain fried rn
        let mut when = now;
        loop {
            if let Some(bus) = buses.iter().find(|n| when % **n == 0) {
                println!("now: {}, when: {}, bus: {}", now, when, bus);
                return Ok(((when - now) * bus).to_string());
            }
            when += 1;
        }
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}
