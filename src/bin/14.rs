use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/14.txt");
    let mut parts = input.split("\n\n");
    let mut template = String::from(parts.next().unwrap());
    let rules: HashMap<&str, String> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let from = parts.next().unwrap();
            let to_char = parts.next().unwrap();
            let mut to = String::from(from);
            to.replace_range(1..2, to_char);
            (from, to)
        })
        .collect();

    for _i in 0..10 {
        template = apply(&template, &rules);
    }

    let mut char_freqs: HashMap<char, u64> = HashMap::with_capacity(26);
    for ch in template.chars() {
        let entry = char_freqs.entry(ch).or_default();
        *entry += 1;
    }

    let mut max = 0u64;
    let mut min = u64::MAX;
    for (_ch, freq) in char_freqs.into_iter() {
        if freq > max {
            max = freq;
        }
        if freq < min {
            min = freq;
        }
    }
    println!("Part 1: {}", max - min);

    for i in 0..30 {
        println!("{}", i);
        template = apply(&template, &rules);
    }

    let mut char_freqs: HashMap<char, u64> = HashMap::with_capacity(26);
    for ch in template.chars() {
        let entry = char_freqs.entry(ch).or_default();
        *entry += 1;
    }

    let mut max = 0u64;
    let mut min = u64::MAX;
    for (_ch, freq) in char_freqs.into_iter() {
        if freq > max {
            max = freq;
        }
        if freq < min {
            min = freq;
        }
    }
    println!("Part 2: {}", max - min);
    Ok(())
}

fn apply(template: &str, rules: &HashMap<&str, String>) -> String {
    let mut result = template
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|wnd| {
            let bin = wnd.iter().collect::<String>();
            let x: &str = &bin;
            rules.get(&x).cloned().unwrap()
        })
        .collect::<Vec<String>>()
        .join("");
    result.push(template.chars().last().unwrap());
    result
}
