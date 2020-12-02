use std::{io, env};

mod challenge;

fn main() -> io::Result<()> {
    let challenge_name: String = env::args().nth(1).expect("pass a challenge name please");
    
    use challenge::*;
    match challenge_name.as_ref() {
        "1a" => d1::part_a(),
        "1b" => d1::part_b(),
        "2a" => d2::part_a(),
        "2b" => d2::part_b(),
        unknown => panic!("unknown challenge \"{}\"", unknown)
    }
}