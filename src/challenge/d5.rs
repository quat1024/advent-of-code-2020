use std::{ops::RangeInclusive, str::FromStr};

use crate::*;
pub struct Challenge5;

struct Seat {
    row: usize,
    col: usize,
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
        *range.start()..=middle
    } else {
        middle + 1..=*range.end()
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
                etc => {
                    return Err(ChallengeErr::Unspecified(format!(
                        "unknown character {}",
                        etc
                    )))
                }
            }
        }

        let row: usize;
        let col: usize;

        if *row_range.start() == *row_range.end() {
            row = *row_range.start();
        } else {
            return Err(ChallengeErr::Unspecified(format!(
                "open range after iteration: {:?}",
                row_range
            )));
        }

        if *col_range.start() == *col_range.end() {
            col = *col_range.start();
        } else {
            return Err(ChallengeErr::Unspecified(format!(
                "open range after iteration: {:?}",
                col_range
            )));
        }

        Ok(Seat { row, col })
    }
}

impl Challenge for Challenge5 {
    fn filename(&self) -> &'static str {
        "5a.txt"
    }

    //Wrong: 749
    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        //TODO wrong way to do error handling lol
        //i know there's something about vec of results -> result of vecs but i couldnt get it working
        let seats: Vec<Seat> = input
            .lines()
            .map(|line| Seat::from_str(line).expect("uh oh"))
            .collect();

        seats
            .iter()
            .map(Seat::id)
            .max()
            .ok_or_else(|| ChallengeErr::Unspecified("no seats lol".into()))
            .map(|u| u.to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        //For some reason rust-analyzer is very very confused by this code if i do not fully qualify HashMap? maybe a bug
        //use std::collections::HashMap;

        let seats: Vec<Seat> = input
            .lines()
            .map(|line| Seat::from_str(line).expect("uh oh"))
            .collect();
        let seats_by_id: std::collections::HashMap<usize, Seat> =
            seats.into_iter().map(|seat| (seat.id(), seat)).collect();
        let max_id = seats_by_id.keys().max().expect("no seats lol");

        //This is not particularly pretty but works.
        //Not sure why i have to borrow a bajillion times
        for i in 0..=max_id - 3 {
            if seats_by_id.contains_key(&i)
                && !seats_by_id.contains_key(&(i + 1))
                && seats_by_id.contains_key(&(i + 2))
            {
                println!("candidate: {}", i + 1);
            }
        }

        //Also it's written with a println cuz the question says that my seat is "not at the very front or back"
        //and i took that to mean "not the first or last row", which would mean i'd have to filter those out
        //(and i wanted to just eyeball it instead of actually writing a filterer)
        //but I think it actually meant that it would not be seat 0 or seat max

        Ok("egg".into())
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
