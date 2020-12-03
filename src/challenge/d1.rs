use crate::*;

pub struct Challenge1;

impl Challenge for Challenge1 {
    fn filename(&self) -> &'static str {
        "1a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let numbers: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for (index, n1) in numbers.iter().enumerate() {
            for n2 in numbers[index..].iter() {
                if n1 + n2 == 2020 {
                    return Ok(format!("product is {}", n1 * n2));
                }
            }
        }

        Err(ChallengeErr::NoSolution("could not find product".into()))
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let numbers: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for (index1, n1) in numbers.iter().enumerate() {
            for (index2, n2) in numbers[index1..].iter().enumerate() {
                for n3 in numbers[index2..].iter() {
                    if (n1 + n2 + n3) == 2020 {
                        return Ok(format!("product is {}", (n1 * n2 * n3)));
                    }
                }
            }
        }

        Err(ChallengeErr::NoSolution("could not find product".into()))
    }
}
