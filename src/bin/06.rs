use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/06.txt");
    let mut fishes = input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    const DAYS: u32 = 80;
    for _i in 0..DAYS {
        let mut new_fishes = 0u32;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                new_fishes += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        for _j in 0..new_fishes {
            fishes.push(8);
        }
    }
    println!("Part 01: {}", fishes.len());

    let mut offsets: HashMap<u32, u64> = HashMap::with_capacity(6);
    for i in input.split(',').map(|x| x.parse::<u32>().unwrap()) {
        let entry = offsets.entry(i).or_default();
        *entry += 1;
    }

    let mut in_two_days = 0;
    let mut in_one_day = 0;

    for i in 0..256u32 {
        let day = i % 7;
        let new_fishes = offsets.get(&day).cloned().unwrap_or_default();

        let ent = offsets.entry(day).or_default();
        *ent += in_one_day;

        in_one_day = in_two_days;
        in_two_days = new_fishes;
    }

    let total_fishes: u64 = offsets.values().sum();
    println!("Part 02: {}", total_fishes + &in_two_days + &in_one_day);

    Ok(())
}
