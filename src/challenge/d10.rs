use crate::*;

pub struct Challenge10;

impl Challenge for Challenge10 {
    fn filename(&self) -> &'static str {
        "10a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut nums = input.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        nums.push(0);
        nums.push(nums.iter().max().unwrap() + 3);
        nums.sort();
        
        let one = nums.array_windows::<2>().filter(|&[a, b]| *b - *a == 1).count();
        let three = nums.array_windows::<2>().filter(|&[a, b]| *b - *a == 3).count();
        
        Ok((one * three).to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}