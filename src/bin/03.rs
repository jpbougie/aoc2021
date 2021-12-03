use std::io::Read;

use anyhow::Result;

fn main() -> Result<()> {
    let mut f = std::fs::File::open("inputs/03.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let lines = s.lines().collect::<Vec<_>>();

    let bits = lines[0].len();

    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for i in 0..bits {
        gamma *= 2;
        epsilon *= 2;
        let ones = lines
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count();

        if dbg!(ones) >= lines.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("Part 1: {}", gamma as u32 * epsilon as u32);

    let mut remaining = lines.clone();

    for i in 0..bits {
        if remaining.len() == 1 {
            break;
        }
        let ones = remaining
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count();

        if 2 * ones >= remaining.len() {
            remaining = remaining
                .into_iter()
                .filter(|b| b.chars().nth(i).unwrap() == '1')
                .collect();
        } else {
            remaining = remaining
                .into_iter()
                .filter(|b| b.chars().nth(i).unwrap() == '0')
                .collect();
        }
    }

    let oxygen_rating = u32::from_str_radix(remaining.get(0).unwrap(), 2)?;

    let mut remaining = lines.clone();

    for i in 0..bits {
        if remaining.len() == 1 {
            break;
        }
        let ones = remaining
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count();

        if 2 * ones >= remaining.len() {
            remaining = remaining
                .into_iter()
                .filter(|b| b.chars().nth(i).unwrap() == '0')
                .collect();
        } else {
            remaining = remaining
                .into_iter()
                .filter(|b| b.chars().nth(i).unwrap() == '1')
                .collect();
        }
    }

    let scrubber_rating = u32::from_str_radix(remaining.get(0).unwrap(), 2)?;

    println!("Part 2: {}", oxygen_rating * scrubber_rating);

    Ok(())
}
