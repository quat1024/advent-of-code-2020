use std::{io, fs};

pub fn part_a() -> io::Result<()> {
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

pub fn part_b() -> io::Result<()> {
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