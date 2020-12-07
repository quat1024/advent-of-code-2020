use crate::*;

pub struct Challenge7;

impl Challenge for Challenge7 {
    fn filename(&self) -> &'static str {
        "7a.txt"
    }

    fn part_a(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}