use std::convert::TryInto;

use crate::*;

pub struct Challenge11;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Board {
    fn from_string(input: String) -> Board {
        let tiles = input
            .lines()
            .map(|line| -> Vec<Tile> {
                line.chars()
                    .map(|c| -> Tile {
                        match c {
                            '.' => Tile::Floor,
                            'L' => Tile::EmptySeat,
                            _ => panic!(),
                        }
                    })
                    .collect()
            })
            .collect::<Vec<_>>();

        let width = tiles[0].len();
        let height = tiles.len();

        Board {
            tiles,
            width,
            height,
        }
    }

    fn step(&mut self) -> bool {
        static ADJACENCY: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut new_tiles = self.tiles.clone();
        let mut changed_anything = false;

        for row in 0..self.height {
            for col in 0..self.width {
                let here = self.tiles[row][col];

                if here == Tile::Floor {
                    continue;
                }

                let neighborhood: Vec<Tile> = ADJACENCY
                    .iter()
                    .map(|&(drow, dcol)| (row as isize + drow, col as isize + dcol))
                    .map(|(row, col)| -> Option<Tile> {
                        let r: usize = row.try_into().ok()?; //discards too-low coordinates
                        let c: usize = col.try_into().ok()?;
                        Some(*self.tiles.get(r)?.get(c)?) //discards too-high coordinates
                    })
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect();

                //TODO find out why i need the double dereference here :thinking:
                let occupied_count = neighborhood
                    .iter()
                    .filter(|&&t| t == Tile::OccupiedSeat)
                    .count();

                //println!("{:?} - {:?}: empty {}, occupied {}", here, neighborhood, empty_count, occupied_count);

                if here == Tile::EmptySeat && occupied_count == 0 {
                    new_tiles[row][col] = Tile::OccupiedSeat;
                    changed_anything = true;
                    continue;
                }

                if here == Tile::OccupiedSeat && occupied_count >= 4 {
                    new_tiles[row][col] = Tile::EmptySeat;
                    changed_anything = true;
                    continue;
                }
            }
        }

        self.tiles = new_tiles;
        changed_anything
    }

    fn count_occupied(&self) -> usize {
        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.tiles[row][col] == Tile::OccupiedSeat {
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        println!();
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.tiles[row][col] {
                        Tile::EmptySeat => 'L',
                        Tile::Floor => '.',
                        Tile::OccupiedSeat => '#',
                    }
                );
            }
            println!();
        }
    }
}

impl Challenge for Challenge11 {
    fn filename(&self) -> &'static str {
        "11a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut b = Board::from_string(input);

        while let true = b.step() {}

        Ok(b.count_occupied().to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}
