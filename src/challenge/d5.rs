use std::{str::FromStr, ops::RangeInclusive};

use crate::*;
pub struct Challenge5;

struct Seat {
    row: usize,
    col: usize
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn bisect_range(range: &RangeInclusive<usize>, small_half: bool) -> RangeInclusive<usize> {
    let middle = (range.end() + range.start()) / 2;
    
    //println!("middle of range {:?} is {}", *range, middle);
    
    if small_half {
        *range.start() ..= middle
    } else {
        middle + 1 ..= *range.end()
    }
}

impl FromStr for Seat {
    type Err = ChallengeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_range = 0..=127;
        let mut col_range = 0..=7;
        
        for c in s.chars() {
            match c {
                'F' => row_range = bisect_range(&row_range, true),
                'B' => row_range = bisect_range(&row_range, false),
                'L' => col_range = bisect_range(&col_range, true),
                'R' => col_range = bisect_range(&col_range, false),
                etc => return Err(ChallengeErr::Unspecified(format!("unknown character {}", etc)))
            }
        }
        
        let row: usize;
        let col: usize;
        
        if *row_range.start() == *row_range.end() {
            row = *row_range.start();
        } else {
            return Err(ChallengeErr::Unspecified(format!("open range after iteration: {:?}", row_range)))
        }
        
        if *col_range.start() == *col_range.end() {
            col = *col_range.start();
        } else {
            return Err(ChallengeErr::Unspecified(format!("open range after iteration: {:?}", col_range)))
        }
        
        Ok(Seat{row, col})
    }
}

impl Challenge for Challenge5 {
    fn filename(&self) -> &'static str {
        "5a.txt"
    }

    //Wrong: 749
    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        //TODO wrong way to do error handling lol
        let seats: Vec<Seat> = input.lines().map(|line| Seat::from_str(line).expect("uh oh")).collect();
        
        seats.iter().map(Seat::id).max().ok_or_else(|| ChallengeErr::Unspecified("no seats lol".into())).map(|u| u.to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}

mod test {
    use super::*;

    #[test]
    fn bisecting_works() {
        assert_eq!(bisect_range(&(0..=127), true), 0..=63);
        assert_eq!(bisect_range(&(0..=127), false), 64..=127);
    }
    
    #[test]
    fn examples() {
        assert_eq!(Seat::from_str("BFFFBBFRRR").unwrap().id(), 567);
        assert_eq!(Seat::from_str("FFFBBBFRRR").unwrap().id(), 119);
        assert_eq!(Seat::from_str("BBFFBBFRLL").unwrap().id(), 820);
    }
}