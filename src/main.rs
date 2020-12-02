use std::{io, env, fs};

fn main() -> io::Result<()> {
    //i will organize this better later shhhhHHH
    
    let challenge_name: String = env::args().nth(1).expect("pass a challenge name please");
    
    match challenge_name.as_ref() {
        "1a" => challenge1a(),
        "1b" => challenge1b(),
        "2a" => challenge2a(),
        "2b" => challenge2b(),
        unknown => panic!("unknown challenge \"{}\"", unknown)
    }
}

fn challenge1a() -> io::Result<()> {
    let numbers: Vec<i32> = fs::read_to_string("1a.txt")?.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
    
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

fn challenge1b() -> io::Result<()> {
    let numbers: Vec<i32> = fs::read_to_string("1a.txt")?.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
    
    //TODO: This doesn't seem correct, it returns the result 3 times lol
    for (index1, n1) in numbers.iter().enumerate() {
        for (index2, n2) in numbers[index1..].iter().enumerate() {
            for n3 in numbers[index2..].iter() {
                if (n1 + n2 + n3) == 2020 {
                    println!("{} {} {}", n1, n2, n3);
                    println!("product is {}", (n1 * n2 * n3));
                } 
            }
        }
    }
    
    Ok(())
}

#[derive(Debug)]
struct PasswordEntry {
    min: i32,
    max: i32,
    letter: char,
    pass: String
}

fn parse_passwords<C>(input: String) -> C where C: std::iter::FromIterator<PasswordEntry> {
    input.lines().map(|line| -> PasswordEntry {
        //Oh no it sucks
        let line_split: Vec<&str> = line.split(' ').collect();
        let policy_split: Vec<i32> = line_split.get(0).expect("policy").split('-').map(|x| x.parse().expect("number in policy")).collect();
        let min = *policy_split.get(0).expect("policy min");
        let max = *policy_split.get(1).expect("policy max");
        let letter = line_split.get(1).expect("letter").chars().next().expect("a char");
        let pass = line_split.get(2).expect("password").to_string();
        
        PasswordEntry{min, max, letter, pass}
    }).collect()
}

fn challenge2a() -> io::Result<()> {
    let input = fs::read_to_string("2a.txt")?;
    let entries: Vec<PasswordEntry> = parse_passwords(input);
    
    let count = entries.iter().filter(|entry| -> bool {
        let letter_count = entry.pass.chars().filter(|c| *c == entry.letter).count() as i32;
        letter_count >= entry.min && letter_count <= entry.max
    }).count();
    
    println!("{}", count);
    
    Ok(())
}

//wrong: 407 (off by one)
//wrong: 423 (off by two)
fn challenge2b() -> io::Result<()> {
    let input = fs::read_to_string("2a.txt")?;
    let entries: Vec<PasswordEntry> = parse_passwords(input);
    
    let count = entries.iter().filter(|entry| -> bool {
        let indexable_pass: Vec<char> = entry.pass.chars().collect();
        indexable_pass.get(entry.min as usize - 1).map(|c| *c == entry.letter).unwrap() ^ indexable_pass.get(entry.max as usize - 1).map(|c| *c == entry.letter).unwrap()
    }).count();
    
    println!("{}", count);
    
    Ok(())
}