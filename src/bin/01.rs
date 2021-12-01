use std::io::{self, Read};
fn main() -> io::Result<()> {
    let mut f = std::fs::File::open("inputs/01.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let lines = s.lines().collect::<Vec<&str>>();
    let depths = lines
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let n = depths.iter().skip(1);
    let greater = depths
        .iter()
        .zip(n)
        .filter(|&(x, y)| x < y)
        .collect::<Vec<_>>();

    println!("Part 1: {}", greater.len());

    let windows = depths
        .windows(3)
        .map(|x| x.iter().sum::<i64>())
        .collect::<Vec<_>>();

    println!(
        "Part 02: {}",
        windows
            .iter()
            .zip(windows.iter().skip(1))
            .filter(|&(x, y)| x < y)
            .count()
    );
    Ok(())
}
