use crate::*;
pub struct Challenge13;

impl Challenge for Challenge13 {
    fn filename(&self) -> &'static str {
        "13a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut lines = input.lines();
        let now: usize = lines.next().unwrap().parse().unwrap();
        let buses: Vec<usize> = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|f| f.parse().ok())
            .collect();

        //There's probably a nice way to do this but brain fried rn
        let mut when = now;
        loop {
            if let Some(bus) = buses.iter().find(|n| when % **n == 0) {
                println!("now: {}, when: {}, bus: {}", now, when, bus);
                return Ok(((when - now) * bus).to_string());
            }
            when += 1;
        }
    }

    //wrong: 745986593211843
    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        struct Bus {
            offset: i128,
            interval: i128,
        }

        let mut constraints: Vec<Bus> = input
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(offset, interval)| {
                Some(Bus {
                    offset: offset as i128,
                    interval: interval.parse::<i128>().ok()?, //discards the 'x' entries
                })
            })
            .collect();
        
        constraints.sort_by_key(|t| t.interval);
        constraints.reverse();
        
        //Ok, time to do the Chinese Remainder Theorem.
        //It "just so happens" that all the input values are prime numbers, wonder why that is :thinking:
        
        //Per https://brilliant.org/wiki/chinese-remainder-theorem/
        let big_n = constraints.iter().map(|bus| bus.interval).product::<i128>();
        
        let x: i128 = constraints.iter().map(|bus| -> i128 {
            let yi = big_n / bus.interval;
            assert_eq!(yi * bus.interval, big_n);
            
            let zi = modular_inverse(yi, bus.interval).unwrap();
            
            bus.offset * yi * zi
        }).sum::<i128>() % big_n;
        
        println!("{}", x);
        
        for bus in constraints.iter() {
            println!("x mod {} === {} (expected {})", bus.interval, x % bus.interval, bus.offset);
        }

        Err(ChallengeErr::NotYetImplemented())
    }
}

fn modular_inverse(number: i128, modulus: i128) -> Option<i128> {
    //Lazy way to do it - signs point to something called "euclid's extended algorithm"
    //which i have not implemented since my numbers are all fairly small, brute force is easy enough
    for i in 0..modulus {
        if (i * number) % modulus == 1 {
            return Some(i);
        }
    }
    
    None
}

mod test {
    use crate::challenge::d13::*;
    
    #[test]
    fn modular_inverse_works() {
        assert_eq!(modular_inverse(1, 4), Some(1));
        assert_eq!(modular_inverse(2, 4), None);
        assert_eq!(modular_inverse(3, 4), Some(3));
        assert_eq!(modular_inverse(6, 59), Some(10));
        assert_eq!(modular_inverse(80, 59), Some(45));
    }
}