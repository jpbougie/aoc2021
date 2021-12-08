use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/08.txt");
    let lines = input
        .lines()
        .map(|line| {
            let parts = line.split(" | ").collect::<Vec<_>>();
            let unique_signal_patterns = parts[0].split(' ').collect::<Vec<_>>();
            let output_value = parts[1].split(' ').collect::<Vec<_>>();
            (unique_signal_patterns, output_value)
        })
        .collect::<Vec<_>>();

    let part1: usize = lines
        .iter()
        .map(|(_x, value)| {
            value
                .iter()
                .filter(|digit| {
                    let l = digit.len();
                    l == 2 || l == 3 || l == 4 || l == 7
                })
                .count()
        })
        .sum();

    println!("Part 1: {}", part1);
    Ok(())
}
