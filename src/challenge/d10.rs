use crate::*;
use itertools::Itertools;

pub struct Challenge10;

impl Challenge for Challenge10 {
    fn filename(&self) -> &'static str {
        "10a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut nums = input
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        nums.push(0);
        nums.push(nums.iter().max().unwrap() + 3);
        nums.sort();

        let one = nums
            .array_windows::<2>()
            .filter(|&[a, b]| *b - *a == 1)
            .count();
        let three = nums
            .array_windows::<2>()
            .filter(|&[a, b]| *b - *a == 3)
            .count();

        Ok((one * three).to_string())
    }

    //wrong: 92264062500
    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut nums = input
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        nums.push(0);
        nums.push(nums.iter().max().unwrap() + 3);
        nums.sort();

        //The trick is that when there's a difference of 3, the only way to cross the gap is via that difference.
        //So I can split on differences of 3, solve the problem for each subgroup, and multiply the results.

        //Also! There are no differences of size 2 anywhere in the input. Only 1 and 3

        let mut product = 1 as usize;
        let lookup = [1, 1, 2, 4, 7]; //created by manually solving the problem
        let mut run_start = 0;
        for (idx, (a, b)) in nums.iter().tuple_windows().enumerate() {
            if b - a == 3 {
                product *= lookup[(idx - run_start)];
                run_start = idx + 1;
            }
        }

        Ok(product.to_string())
    }
}
