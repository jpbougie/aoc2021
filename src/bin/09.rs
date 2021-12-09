use std::collections::{HashSet, VecDeque};

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/09.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut part1 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if neighbours(x, y, width, height)
                .into_iter()
                .all(|(nx, ny)| grid[ny][nx] > *cell)
            {
                part1 += 1 + cell;
            }
        }
    }

    println!("Part 1: {}", part1);

    let mut basins = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if neighbours(x, y, width, height)
                .into_iter()
                .all(|(nx, ny)| grid[ny][nx] > *cell)
            {
                let mut points = HashSet::new();
                points.insert((x, y));
                basins.push(points);
            }
        }
    }

    basins.iter_mut().for_each(|b| {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(*b.iter().next().unwrap());
        while let Some((nx, ny)) = to_visit.pop_front() {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            b.insert((nx, ny));
            visited.insert((nx, ny));
            let mut n = neighbours(nx, ny, width, height);
            n.retain(|&(x, y)| grid[y][x] != 9);
            for p in n.into_iter() {
                to_visit.push_back(p);
            }
        }
    });

    let mut lens: Vec<usize> = basins.into_iter().map(|b| b.len()).collect();
    lens.sort();
    lens.reverse();
    let part2: usize = lens.into_iter().take(3).product();
    println!("Part 2: {}", part2);
    Ok(())
}

fn neighbours(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if y > 0 {
        n.push((x, y - 1));
    }

    if x > 0 {
        n.push((x - 1, y));
    }

    if x < width - 1 {
        n.push((x + 1, y));
    }

    if y < height - 1 {
        n.push((x, y + 1));
    }
    n
}
