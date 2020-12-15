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

                println!("mask_mask: {:#036b}, mask: {:#036b}", mask_mask, mask);
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
        //STILL doesn't work, i'm busy on day 15 rn :)

        let mut mem: HashMap<u64, u64> = HashMap::new();

        enum MaskBit {
            Passthrough,
            One,
            Floating,
        }

        //left side of this thing is the high bits
        let mut mask: Vec<MaskBit> = Vec::with_capacity(36);

        for line in input.lines() {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                mask.clear();
                mask.extend(mask_str.chars().map(|ch| match ch {
                    '0' => MaskBit::Passthrough,
                    '1' => MaskBit::One,
                    'X' => MaskBit::Floating,
                    _ => panic!(),
                }));
                assert_eq!(mask.len(), 36);
            } else if let Some(rest) = line.strip_prefix("mem[") {
                let mut split = rest.split("] = ");
                let base_addr = split.next().unwrap().parse::<u64>().unwrap();
                let value = split.next().unwrap().parse::<u64>().unwrap();

                let mut unfloating_addr: u64 = 0;
                for idx in 0..=35 {
                    match mask[idx] {
                        MaskBit::Passthrough => unfloating_addr |= ((1 << 35) >> idx) & base_addr,
                        MaskBit::One => unfloating_addr |= (1 << 35) >> idx,
                        _ => (),
                    }
                }
            }
        }

        unimplemented!()
    }
}
