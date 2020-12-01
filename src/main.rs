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
    //Load file
    let file_string = fs::read_to_string("1a.txt")?;
    
    //Parse file
    let numbers: Vec<i32> = file_string
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    //Solve challenge
    for (index, n1) in numbers.iter().enumerate() {
        for n2 in numbers[index..].iter() {
            if n1 + n2 == 2020 {
                println!("product is {}", n1 * n2);
            }
        }
    }
    
    Ok(())
}