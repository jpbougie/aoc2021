use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/15.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    let mut possible_paths = BinaryHeap::new();
    possible_paths.push(Walk {
        cost: 0,
        pos: (0, 0),
        distance: width + height,
    });
    let mut final_path: Option<Walk> = None;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some(walk) = possible_paths.pop() {
        if walk.pos == (width - 1, height - 1) {
            final_path = Some(walk);
            break;
        }
        if visited.contains(&walk.pos) {
            continue;
        }
        visited.insert(walk.pos);
        let neighbours = neighbours(walk.pos.0, walk.pos.1, width, height);
        for neighbour in neighbours {
            let cost = grid[neighbour.1][neighbour.0];
            let distance = height - neighbour.1 + width - neighbour.0;
            possible_paths.push(Walk {
                pos: neighbour,
                cost: walk.cost + cost,
                distance,
            });
        }
    }

    println!("Part 1: {}", final_path.unwrap().cost);

    let mut big_grid: Vec<Vec<u32>> = vec![vec![0; 5 * width]; 5 * height];
    for iter_j in 0..5 {
        for iter_i in 0..5 {
            let base_x = iter_i * width;
            let base_y = iter_j * width;
            let cost_offset = (iter_i + iter_j) as u32;
            for (y, row) in grid.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    let mut new_cost = 1 + (*cell - 1 + cost_offset) % 9;
                    if new_cost == 0 {
                        new_cost = 1;
                    }
                    big_grid[base_y + y][base_x + x] = new_cost;
                }
            }
        }
    }

    let width = big_grid[0].len();
    let height = big_grid.len();
    let mut possible_paths = BinaryHeap::new();
    possible_paths.push(Walk {
        cost: 0,
        pos: (0, 0),
        distance: width + height,
    });
    let mut final_path: Option<Walk> = None;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some(walk) = possible_paths.pop() {
        if walk.pos == (width - 1, height - 1) {
            final_path = Some(walk);
            break;
        }
        if visited.contains(&walk.pos) {
            continue;
        }
        visited.insert(walk.pos);
        let neighbours = neighbours(walk.pos.0, walk.pos.1, width, height);
        for neighbour in neighbours {
            let cost = big_grid[neighbour.1][neighbour.0];
            let distance = height - neighbour.1 + width - neighbour.0;
            possible_paths.push(Walk {
                pos: neighbour,
                cost: walk.cost + cost,
                distance,
            });
        }
    }

    println!("Part 2: {}", final_path.unwrap().cost);

    Ok(())
}

#[allow(unused)]
fn print_grid(big_grid: &[Vec<u32>]) {
    for row in big_grid {
        for cell in row {
            print!("{}", cell);
        }

        println!();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Walk {
    cost: u32,
    distance: usize,
    pos: (usize, usize),
}

impl Ord for Walk {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.distance.cmp(&other.distance))
            .then_with(|| self.pos.0.cmp(&other.pos.0))
            .then_with(|| self.pos.1.cmp(&other.pos.1))
    }
}

impl PartialOrd for Walk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
