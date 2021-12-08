use std::collections::HashSet;

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

    let mut sum = 0usize;
    for (digits, output) in lines {
        let mut patterns: Vec<_> = (0..10).map(|_x| HashSet::new()).collect();
        patterns[8] = segments_with_len(&digits, 7).into_iter().next().unwrap();
        let one = one_segments(&digits);
        patterns[1] = one.clone();
        let seven = seven_segments(&digits);
        patterns[7] = seven.clone();
        let four = four_segments(&digits);
        patterns[4] = four.clone();
        let b_or_d = b_or_d_segments(&four, &one);
        let c_or_f = one.clone();

        let digits_with_five = segments_with_len(&digits, 5);
        let (threes, two_or_five): (Vec<_>, Vec<_>) = digits_with_five
            .into_iter()
            .partition(|digits| digits.intersection(&c_or_f).count() == 2);

        let three = threes.into_iter().next().unwrap();
        patterns[3] = three.clone();

        let (fives, twos): (Vec<_>, Vec<_>) = two_or_five
            .into_iter()
            .partition(|dig| dig.intersection(&b_or_d).count() == 2);

        patterns[5] = fives.into_iter().next().unwrap();
        patterns[2] = twos.into_iter().next().unwrap();

        let c = patterns[3]
            .difference(&patterns[5])
            .cloned()
            .next()
            .unwrap();

        let segments_with_six = segments_with_len(&digits, 6);
        let (sixes, zero_or_nine): (Vec<_>, Vec<_>) = segments_with_six
            .into_iter()
            .partition(|dig| !dig.contains(&c));

        patterns[6] = sixes.into_iter().next().unwrap();
        let (zeros, nines): (Vec<_>, Vec<_>) = zero_or_nine
            .into_iter()
            .partition(|dig| dig.intersection(&b_or_d).count() == 1);

        patterns[0] = zeros.into_iter().next().unwrap();
        patterns[9] = nines.into_iter().next().unwrap();

        let mut value = 0usize;
        for digit in output
            .into_iter()
            .map(|c| c.chars().collect::<HashSet<char>>())
        {
            value *= 10;
            value += patterns.iter().position(|p| p == &digit).unwrap();
        }

        sum += value;
    }

    println!("Part 2: {}", sum);

    Ok(())
}

// Numbers with 2 segments: 1
// Numbers with 3 segments: 7
// Numbers with 4 segments: 4
// Numbers with 5 segments: 2, 3, 5
// Numbers with 6 segments: 0, 6, 9
// Numbers with 7 segments: 8

// a: 8 times
// b: 6 times
// c: 8 times
// d: 7 times
// e: 4 times
// f: 9 times

fn b_or_d_segments(four_segments: &HashSet<char>, one_segments: &HashSet<char>) -> HashSet<char> {
    four_segments.difference(one_segments).cloned().collect()
}

fn one_segments(line: &[&str]) -> HashSet<char> {
    segments_with_len(line, 2).into_iter().next().unwrap()
}

fn seven_segments(line: &[&str]) -> HashSet<char> {
    segments_with_len(line, 3).into_iter().next().unwrap()
}

fn four_segments(line: &[&str]) -> HashSet<char> {
    segments_with_len(line, 4).into_iter().next().unwrap()
}

fn segments_with_len(digits: &[&str], len: usize) -> Vec<HashSet<char>> {
    digits
        .iter()
        .filter_map(|x| {
            if x.len() == len {
                Some(x.chars().collect())
            } else {
                None
            }
        })
        .collect()
}
