use crate::*;

pub struct Challenge6;

impl Challenge for Challenge6 {
    fn filename(&self) -> &'static str {
        "6a.txt"
    }

    fn part_a(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}