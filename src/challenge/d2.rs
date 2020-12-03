use crate::*;

#[derive(Debug)]
struct PasswordEntry {
    min: i32,
    max: i32,
    letter: char,
    pass: String,
}

fn parse_passwords<C>(input: String) -> C
where
    C: std::iter::FromIterator<PasswordEntry>,
{
    input
        .lines()
        .map(|line| -> PasswordEntry {
            //Oh no it sucks
            let line_split: Vec<&str> = line.split(' ').collect();
            let policy_split: Vec<i32> = line_split
                .get(0)
                .expect("policy")
                .split('-')
                .map(|x| x.parse().expect("number in policy"))
                .collect();
            let min = *policy_split.get(0).expect("policy min");
            let max = *policy_split.get(1).expect("policy max");
            let letter = line_split
                .get(1)
                .expect("letter")
                .chars()
                .next()
                .expect("a char");
            let pass = line_split.get(2).expect("password").to_string();

            PasswordEntry {
                min,
                max,
                letter,
                pass,
            }
        })
        .collect()
}

pub struct Challenge2;

impl Challenge for Challenge2 {
    fn filename(&self) -> &'static str {
        "2a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let entries: Vec<PasswordEntry> = parse_passwords(input);

        let count = entries
            .iter()
            .filter(|entry| -> bool {
                let letter_count = entry.pass.chars().filter(|c| *c == entry.letter).count() as i32;
                letter_count >= entry.min && letter_count <= entry.max
            })
            .count();

        Ok(format!("{} valid passwords", count))
    }

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let entries: Vec<PasswordEntry> = parse_passwords(input);

        let count = entries
            .iter()
            .filter(|entry| -> bool {
                let indexable_pass: Vec<char> = entry.pass.chars().collect();
                #[rustfmt::skip] {
                    indexable_pass.get(entry.min as usize - 1).map(|c| *c == entry.letter).unwrap() ^ indexable_pass.get(entry.max as usize - 1).map(|c| *c == entry.letter).unwrap()
                }
            })
            .count();

        Ok(format!("{} valid passwords", count))
    }
}
