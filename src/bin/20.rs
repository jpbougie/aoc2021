use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../inputs/20.txt");
    let mut parts = input.lines();
    let alg = parts
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();

    parts.next().unwrap();

    let mut input_img = Img::new();
    for (y, line) in parts.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                input_img.insert((x as i32, y as i32));
            }
        }
    }

    let mut puzzle = Puzzle {
        img: input_img,
        not: false,
    };

    for _i in 0..2 {
        puzzle = enhance(&puzzle, &alg);
    }

    println!("Part 1: {}", puzzle.img.len());

    for _i in 0..48 {
        puzzle = enhance(&puzzle, &alg);
    }

    println!("Part 2: {}", puzzle.img.len());

    Ok(())
}

type Img = HashSet<(i32, i32)>;

struct Puzzle {
    not: bool,
    img: Img,
}

fn enhance(input_img: &Puzzle, alg: &[bool]) -> Puzzle {
    let bounds = bounds(&input_img.img);
    let mut new_img = Img::new();
    let mut new_not = input_img.not;

    if alg[0] {
        new_not = !new_not;
    }

    for y in (bounds.ymin - 1)..=(bounds.ymax + 1) {
        for x in (bounds.xmin - 1)..=(bounds.xmax + 1) {
            let offset = offset(input_img, x, y);
            if alg[offset] != new_not {
                new_img.insert((x, y));
            }
        }
    }

    Puzzle {
        img: new_img,
        not: new_not,
    }
}

fn offset(input_img: &Puzzle, x: i32, y: i32) -> usize {
    let mut res = 0;

    for y in (y - 1)..=(y + 1) {
        for x in (x - 1)..=(x + 1) {
            res *= 2;
            if input_img.img.contains(&(x, y)) != input_img.not {
                res += 1;
            }
        }
    }

    res
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            xmin: i32::MAX,
            xmax: i32::MIN,
            ymin: i32::MAX,
            ymax: i32::MIN,
        }
    }
}

fn bounds(input_img: &Img) -> Bounds {
    let mut bs: Bounds = Default::default();

    for &(x, y) in input_img {
        if x < bs.xmin {
            bs.xmin = x;
        }

        if x > bs.xmax {
            bs.xmax = x;
        }

        if y < bs.ymin {
            bs.ymin = y;
        }

        if y > bs.ymax {
            bs.ymax = y;
        }
    }

    bs
}
