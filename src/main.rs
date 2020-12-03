#![feature(stmt_expr_attributes)]

use challenge::*;
use chrono::prelude::*;
use std::{env, fs, io, path::PathBuf};

mod challenge;

fn main() -> Result<(), ChallengeErr> {
    //TODO can i make this static at all
    let challenges: Vec<Box<dyn Challenge>> = vec![
        Box::new(d1::Challenge1),
        Box::new(d2::Challenge2),
        Box::new(d3::Challenge3),
    ];

    let challenge_num = env::args()
        .nth(1)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or_else(|| -> usize {
            println!("no challenge number passed (or couldn't parse it), using current date");
            Local::now().day() as usize
        });

    let challenge_num = challenge_num - 1; //zero-index it

    let challenge = &challenges[challenge_num];
    let mut path = env::current_dir()?;
    path.push(["work", challenge.filename()].iter().collect::<PathBuf>());
    println!("{:?}", path);
    let file = fs::read_to_string(path).expect("Unable to read input file");

    //This output could be much, much better
    println!("{:?}", challenge.part_a(file.clone()));
    println!("{:?}", challenge.part_b(file.clone()));
    Ok(())
}

//TODO: beefen this up idk, what are some nice error handling libs? the one with the error deriving sounds nice
#[derive(Debug)]
pub enum ChallengeErr {
    Io(io::Error),
    NotYetImplemented(),
    NoSolution(String),
    Unspecified(String), //Mom can i have extensible errors? We have extensible errors at home
}

impl From<io::Error> for ChallengeErr {
    fn from(e: io::Error) -> Self {
        ChallengeErr::Io(e)
    }
}

pub trait Challenge {
    fn filename(&self) -> &'static str;

    fn part_a(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        Err(ChallengeErr::NotYetImplemented())
    }
}
