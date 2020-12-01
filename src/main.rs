use std::*;

fn main() -> io::Result<()> {
    //i will organize this better later shhhhHHH
    
    let challenge_name: String = env::args().nth(1).expect("pass a challenge name please");
    
    match challenge_name.as_ref() {
        "1a" => challenge1a(),
        unknown => panic!("unknown challenge \"{}\"", unknown)
    }
}

fn challenge1a() -> io::Result<()> {
    Ok(())
}