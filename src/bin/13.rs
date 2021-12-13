use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/13.txt");
    let mut parts = input.split("\n\n");
    let mut dots = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<i64>().unwrap());
            let x = coords.next().unwrap();
            let y = coords.next().unwrap();
            (x, y)
        })
        .collect::<Vec<(i64, i64)>>();

    let instructions = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut instr = line.split('=');
            let left = instr.next().unwrap();
            let axis = left.chars().last().unwrap();
            let coord = instr.next().unwrap().parse().unwrap();
            (axis, coord)
        })
        .collect::<Vec<(char, i64)>>();

    let mut instrs = instructions.iter().cloned();
    fold(&mut dots, instrs.next().unwrap());
    println!(
        "Part 1: {}",
        dots.iter().cloned().collect::<HashSet<(i64, i64)>>().len()
    );

    for instr in instrs {
        fold(&mut dots, instr);
    }

    let (w, h) = bounds(&dots);
    let final_dots: HashSet<(i64, i64)> = dots.iter().cloned().collect::<HashSet<(i64, i64)>>();

    for y in 0..=h {
        for x in 0..=w {
            if final_dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    Ok(())
}

fn bounds(dots: &[(i64, i64)]) -> (i64, i64) {
    let mut max_x = 0i64;
    let mut max_y = 0i64;

    for (x, y) in dots {
        if *x > max_x {
            max_x = *x;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    (max_x, max_y)
}

fn fold(sheet: &mut Vec<(i64, i64)>, instruction: (char, i64)) {
    let (axis, dist) = instruction;
    for dot in sheet.iter_mut() {
        if axis == 'y' && dot.1 > dist {
            dot.1 = dot.1 - 2 * (dot.1 - dist);
        }

        if axis == 'x' && dot.0 > dist {
            dot.0 = dot.0 - 2 * (dot.0 - dist);
        }
    }
}
