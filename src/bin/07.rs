use std::str::FromStr;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/07.txt");
    let positions: Vec<i64> = input
        .split(',')
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()?;

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut best_cost = i64::MAX;

    for proposed_pos in min..=max {
        let mut cost = 0;
        for pos in positions.iter() {
            if cost > best_cost {
                break;
            }

            cost += (proposed_pos - pos).abs();
        }

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("Part 01: {}", best_cost);

    let mut best_cost = i64::MAX;

    for proposed_pos in min..=max {
        let mut cost = 0;
        for pos in positions.iter() {
            if cost > best_cost {
                break;
            }

            cost += second_cost((proposed_pos - pos).abs());
        }

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("Part 01: {}", best_cost);
    Ok(())
}

fn second_cost(dist: i64) -> i64 {
    (1..=dist).sum()
}
