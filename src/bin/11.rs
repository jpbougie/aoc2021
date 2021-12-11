use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/11.txt");
    let original_grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut grid = original_grid.clone();
    let mut flashes = 0;
    for _i in 0..100 {
        flashes += step(&mut grid);
    }

    println!("Part 1: {}", flashes);

    let mut grid = original_grid;
    let mut steps = 0;

    while !fully_lit(&grid) {
        step(&mut grid);
        steps += 1;
    }

    println!("Steps: {}", steps);

    Ok(())
}

fn fully_lit(grid: &[Vec<u32>]) -> bool {
    grid.iter().all(|row| row.iter().all(|c| *c == 0))
}

#[allow(dead_code)]
fn grid_to_str(grid: &[Vec<u32>]) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut flashes = 0;
    let mut to_visit = Vec::new();
    let w = grid.iter().next().unwrap().len();
    let h = grid.len();

    // increase every energy level by 1
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell += 1;
            if *cell == 10 {
                flashes += 1;
                for n in neighbours(x, y, w, h) {
                    to_visit.push(n);
                }
            }
        }
    }

    while !to_visit.is_empty() {
        let visits = to_visit.clone();
        to_visit.clear();
        for (x, y) in visits.into_iter() {
            grid[y][x] += 1;
            if grid[y][x] == 10 {
                flashes += 1;
                for n in neighbours(x, y, w, h) {
                    to_visit.push(n);
                }
            }
        }
    }

    // reset all flashed to 0
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if *cell >= 10 {
                *cell = 0;
            }
        }
    }

    flashes
}

fn neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    // TOP
    if y > 0 {
        n.push((x, y - 1));
    }

    // LEFT
    if x > 0 {
        n.push((x - 1, y));
    }

    // TOP LEFT
    if y > 0 && x > 0 {
        n.push((x - 1, y - 1));
    }

    // RIGHT
    if x < width - 1 {
        n.push((x + 1, y));
    }

    // TOP RIGHT
    if y > 0 && x < width - 1 {
        n.push((x + 1, y - 1));
    }

    // BOTTOM
    if y < height - 1 {
        n.push((x, y + 1));
    }

    // BOTTOM LEFT
    if y < height - 1 && x > 0 {
        n.push((x - 1, y + 1));
    }

    // BOTTOM RIGHT
    if y < height - 1 && x < width - 1 {
        n.push((x + 1, y + 1));
    }
    n
}
