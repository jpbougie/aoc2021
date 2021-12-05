use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/05.txt");
    let lines = input
        .lines()
        .map(|l| {
            let from_to = l
                .split(" -> ")
                .map(|parts| {
                    parts
                        .split(',')
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>();
            Line {
                x1: from_to[0][0],
                y1: from_to[0][1],
                x2: from_to[1][0],
                y2: from_to[1][1],
            }
        })
        .collect::<Vec<_>>();

    let straight_lines = lines
        .iter()
        .filter(|l| l.horizontal() || l.vertical())
        .cloned()
        .collect::<Vec<_>>();

    let mut straight_points: HashMap<(i32, i32), u32> = HashMap::new();
    for line in straight_lines.iter() {
        for p in line.straight_points() {
            let e = straight_points.entry(p).or_default();
            *e += 1;
        }
    }

    let intersections = straight_points.values().filter(|x| **x > 1).count();

    println!("Part 01: {}", intersections);

    let mut points: HashMap<(i32, i32), u32> = HashMap::new();
    for line in lines.iter() {
        for p in line.points() {
            let e = points.entry(p).or_default();
            *e += 1;
        }
    }

    let intersections = points.values().filter(|x| **x > 1).count();

    println!("Part 02: {}", intersections);
    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn vertical(&self) -> bool {
        self.y1 == self.y2
    }

    fn points(&self) -> Vec<(i32, i32)> {
        if self.vertical() {
            let (min_x, max_x) = if self.x1 <= self.x2 {
                (self.x1, self.x2)
            } else {
                (self.x2, self.x1)
            };
            (min_x..=max_x).into_iter().map(|x| (x, self.y1)).collect()
        } else if self.horizontal() {
            let (min_y, max_y) = if self.y1 <= self.y2 {
                (self.y1, self.y2)
            } else {
                (self.y2, self.y1)
            };
            (min_y..=max_y).into_iter().map(|y| (self.x1, y)).collect()
        } else {
            // 45 degrees angle
            let slope_x = if (self.x2 - self.x1) > 0 { 1 } else { -1 };
            let slope_y = if (self.y2 - self.y1) > 0 { 1 } else { -1 };

            let mut points = Vec::new();

            let mut x = self.x1;
            let mut y = self.y1;
            loop {
                points.push((x, y));
                if x == self.x2 && y == self.y2 {
                    break;
                }
                x += slope_x;
                y += slope_y;
            }

            points
        }
    }

    fn straight_points(&self) -> Vec<(i32, i32)> {
        if self.vertical() {
            let (min_x, max_x) = if self.x1 <= self.x2 {
                (self.x1, self.x2)
            } else {
                (self.x2, self.x1)
            };
            (min_x..=max_x).into_iter().map(|x| (x, self.y1)).collect()
        } else {
            let (min_y, max_y) = if self.y1 <= self.y2 {
                (self.y1, self.y2)
            } else {
                (self.y2, self.y1)
            };
            (min_y..=max_y).into_iter().map(|y| (self.x1, y)).collect()
        }
    }
}
