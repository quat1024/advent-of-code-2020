use std::convert::{TryFrom, TryInto};

use crate::*;

pub struct Challenge3;

struct Map {
    trees: Vec<Vec<Entry>>, //row-major; are there any better structures available? (yes)
}

impl Map {
    fn get(&self, row: usize, column: usize) -> Option<Entry> {
        let row = self.trees.get(row)?;
        Some(row[column % row.len()])
    }

    fn count_along_slope(&self, rowd: usize, cold: usize) -> usize {
        //This smells like something an iterator can do better, lol
        let mut tree_count = 0;
        let mut row = 0;
        let mut col = 0;

        let mut op: Option<Entry> = Some(Entry::Air);
        while let Some(e) = op {
            if e == Entry::Tree {
                tree_count += 1;
            }
            row += rowd;
            col += cold;
            op = self.get(row, col);
        }

        tree_count
    }
}

impl TryFrom<String> for Map {
    type Error = ChallengeErr;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        //TODO: This feels like something that can start with "input.lines().iter()"
        //However Entry being TryFrom makes it hard. I need the error to bubble up

        let mut rows: Vec<Vec<Entry>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<Entry> = Vec::new();

            for c in line.chars() {
                row.push(Entry::try_from(c)?); // ?
            }

            rows.push(row);
        }

        Ok(Map { trees: rows })
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Entry {
    Air,
    Tree,
}

impl TryFrom<char> for Entry {
    type Error = ChallengeErr;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Entry::Air),
            '#' => Ok(Entry::Tree),
            etc => Err(ChallengeErr::Unspecified(
                format!("unknown character {}", etc).into(),
            )),
        }
    }
}

impl Challenge for Challenge3 {
    fn filename(&self) -> &'static str {
        "3a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        Ok(Map::try_from(input)?.count_along_slope(1, 3).to_string())
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let map: Map = input.try_into()?;

        Ok([(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|(rowd, cold)| map.count_along_slope(*rowd, *cold))
            .product::<usize>()
            .to_string())
    }
}
