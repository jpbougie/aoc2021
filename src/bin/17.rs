use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/17.txt");
    let re = Regex::new(
        r"target area: x=(?P<xmin>-?\d+)\.\.(?P<xmax>-?\d+), y=(?P<ymin>-?\d+)..(?P<ymax>-?\d+)",
    )
    .unwrap();
    let caps = re.captures(input).unwrap();
    let (xmin, xmax) = (
        (&caps["xmin"]).parse::<i64>().unwrap(),
        (&caps["xmax"]).parse::<i64>().unwrap(),
    );
    let (ymin, ymax) = (
        (&caps["ymin"]).parse::<i64>().unwrap(),
        (&caps["ymax"]).parse::<i64>().unwrap(),
    );

    let bounds = Bounds {
        xmin,
        xmax,
        ymin,
        ymax,
    };

    let mut max_y = 0i64;
    let mut valids_runs = 0u64;

    for initial_xvelocity in 1..=(xmax + 1) {
        for initial_yvelocity in (ymin - 1)..1000 {
            let mut x = 0i64;
            let mut y = 0i64;
            let mut run_maxy = 0i64;
            let mut valid_run = false;
            let mut velx = initial_xvelocity;
            let mut vely = initial_yvelocity;
            loop {
                if bounds.contains(x, y) {
                    valid_run = true;
                }

                if y > run_maxy {
                    run_maxy = y;
                }

                if bounds.overshot(x, y) {
                    break;
                }
                x += velx;
                y += vely;

                if velx > 0 {
                    velx -= 1;
                }

                vely -= 1;
            }

            if valid_run {
                valids_runs += 1;
            }
            if valid_run && run_maxy > max_y {
                max_y = run_maxy;
            }
        }
    }

    println!("Part 1: {}", max_y);
    println!("Part 2: {}", valids_runs);

    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
struct Bounds {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Bounds {
    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }

    fn overshot(&self, x: i64, y: i64) -> bool {
        x > self.xmax || y < self.ymin
    }
}
