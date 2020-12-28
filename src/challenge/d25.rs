use crate::*;

pub struct Challenge25;

impl Challenge for Challenge25 {
    fn filename(&self) -> &'static str {
        "25a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let (card_public, door_public) = {
            let mut lines = input.lines();
            (
                lines.next().unwrap().parse::<u64>().unwrap(),
                lines.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        println!("Working");

        let (card_loop, door_loop) = (
            bruteforce_loop_size(7, card_public),
            bruteforce_loop_size(7, door_public),
        );
        println!("card loop: {}, door loop: {}", card_loop, door_loop);

        let shared_key = bruteforce_forwards_work(card_public, door_loop);
        assert_eq!(shared_key, bruteforce_forwards_work(door_public, card_loop));

        Ok(shared_key.to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        //I forgot that day 25 doesn't have a part 2!
        Ok("Merry Christmas and Happy Holidays!".to_owned())
    }
}

fn bruteforce_loop_size(subject: u64, public: u64) -> u64 {
    let mut value = 1;
    let mut loop_count = 1;
    loop {
        value *= subject;
        value %= 20201227;
        if value == public {
            return loop_count;
        }
        loop_count += 1;
    }
}

fn bruteforce_forwards_work(subject: u64, loop_size: u64) -> u64 {
    let mut value = subject;
    for _ in 1..loop_size {
        value *= subject;
        value %= 20201227;
    }
    value
}
