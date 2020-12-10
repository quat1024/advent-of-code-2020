use crate::*;

pub struct Challenge10;

impl Challenge for Challenge10 {
    fn filename(&self) -> &'static str {
        "10a.txt"
    }

    fn part_a(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}