use crate::*;

pub struct Challenge15;

impl Challenge for Challenge15 {
    fn filename(&self) -> &'static str {
        "15a.txt"
    }

    fn part_a(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}