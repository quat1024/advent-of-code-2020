use std::collections::HashMap;

use crate::*;
pub struct Challenge14;

impl Challenge for Challenge14 {
    fn filename(&self) -> &'static str {
        "14a.txt"
    }

    fn part_a(&self, input: String) -> Result<String, ChallengeErr> {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut mask: u64 = 0;
        let mut mask_mask: u64 = 0;
        
        for line in input.lines() {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                //When the mask is not X, put a 1 in that bit position. 
                mask_mask = mask_str.chars().enumerate().map(|(idx, ch)| match ch {
                    'X' => 0,
                    _ => (1 << 35) >> idx
                }).fold(0, |a, b| a | b);
                
                mask = mask_str.chars().enumerate().map(|(idx, ch)| match ch {
                    '1' => (1 << 35) >> idx,
                    _ => 0
                }).fold(0, |a, b| a | b);
                
                println!("mask_mask: {:#036b}, mask: {:#036b}", mask_mask, mask);
            } else if let Some(rest) = line.strip_prefix("mem["){
                let mut split = rest.split("] = "); //jank
                let addr = split.next().unwrap().parse::<u64>().unwrap();
                let value = split.next().unwrap().parse::<u64>().unwrap();
                
                let masked_mask = mask & mask_mask;
                let unmasked_value = value & (!mask_mask);
                mem.insert(addr, masked_mask | unmasked_value);
            }
        }
        
        Ok(mem.values().sum::<u64>().to_string())
    }

    fn part_b(&self, _input: String) -> Result<String, ChallengeErr> {
        //Good night!
        unimplemented!()
    }
}