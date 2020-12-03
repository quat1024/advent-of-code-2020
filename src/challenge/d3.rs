use std::convert::TryFrom;

use crate::*;

pub struct Original;

struct Map {
    trees: Vec<Vec<Entry>> //row-major; are there any better structures available? (yes)
}

impl Map {
    fn get(&self, row: usize, column: usize) -> Option<Entry> {
        let row = self.trees.get(row)?;
        Some(row[column % row.len()])
    }
}

//TODO this is WRONG, use TryFrom
impl From<String> for Map {
    fn from(input: String) -> Self {
        Map {
            trees: input.lines().map(|x| x.chars().map(|c| Entry::from(c)).collect()).collect()
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Entry {
    Air,
    Tree
}

//TODO this is WRONG, use TryFrom
impl From<char> for Entry {
    fn from(c: char) -> Self {
        match c {
            '.' => Entry::Air,
            '#' => Entry::Tree,
            etc => panic!("we have TryFrom at home... unknown char {}", etc)
        }
    }
}

impl Challenge for Original {
    fn filename(&self) -> &'static str {
        "3a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let map: Map = input.into(); //todo tryInto
        
        //This smells like something an iterator can do better, lol
        let mut tree_count = 0;
        let mut row = 0;
        let mut col = 0;
        
        let mut op: Option<Entry> = Some(Entry::Air);
        while let Some(e) = op {
            if e == Entry::Tree { tree_count += 1; }
            row += 1;
            col += 3;
            op = map.get(row, col);
        }
        
        Ok(tree_count.to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}