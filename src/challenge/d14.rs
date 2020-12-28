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
                mask_mask = mask_str
                    .chars()
                    .enumerate()
                    .map(|(idx, ch)| match ch {
                        'X' => 0,
                        _ => (1 << 35) >> idx,
                    })
                    .fold(0, |a, b| a | b);

                mask = mask_str
                    .chars()
                    .enumerate()
                    .map(|(idx, ch)| match ch {
                        '1' => (1 << 35) >> idx,
                        _ => 0,
                    })
                    .fold(0, |a, b| a | b);

                //println!("mask_mask: {:#036b}, mask: {:#036b}", mask_mask, mask);
            } else if let Some(rest) = line.strip_prefix("mem[") {
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

    fn part_b(&self, input: String) -> Result<String, ChallengeErr> {
        let mut mem: HashMap<u64, u64> = HashMap::new();

        let mut masks: Vec<u64> = Vec::new();
        let mut mask_show_through: u64 = 0;

        for line in input.lines() {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                let chars = mask_str.chars().collect::<Vec<_>>();
                masks.clear();
                masks.push(0);
                mask_show_through = 0;

                for char_idx in 0..=35 {
                    let ch = chars[char_idx];
                    let bit = (1 << 35) >> char_idx;
                    match ch {
                        '0' => (),
                        '1' => masks.iter_mut().for_each(|m| *m |= bit),
                        'X' => {
                            let mut masks_copy = masks.clone();
                            masks_copy.iter_mut().for_each(|m| *m |= bit);
                            masks.extend(masks_copy);

                            mask_show_through |= bit;
                        }
                        _ => panic!(),
                    }
                }
            } else if let Some(rest) = line.strip_prefix("mem[") {
                let mut split = rest.split("] = ");
                let base_addr = split.next().unwrap().parse::<u64>().unwrap();
                let value = split.next().unwrap().parse::<u64>().unwrap();

                //println!("base_addr: {:#036b}", base_addr);
                for mask in masks.iter() {
                    let a = mask & mask_show_through;
                    let b = (mask | base_addr) & !(mask_show_through);
                    let masked_addr = a | b;

                    //println!("masked:    {:#036b}, ({})", masked_addr, masked_addr);
                    mem.insert(masked_addr, value);
                }
            }
        }

        Ok(mem.values().sum::<u64>().to_string())
    }
}
