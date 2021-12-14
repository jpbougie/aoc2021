use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/14.txt");
    let mut parts = input.split("\n\n");
    let original_template = String::from(parts.next().unwrap());
    let mut template = original_template.clone();

    let rules_input = parts.next().unwrap();
    let rules: HashMap<&str, String> = rules_input
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
    println!("Part 1: {}", max + 1 - min);

    let rules: Rules = rules_input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let from = parts.next().unwrap();
            let mut from_chars = from.chars();
            let from_first = from_chars.next().unwrap();
            let from_second = from_chars.next().unwrap();
            let to_char = parts.next().unwrap().chars().next().unwrap();
            (
                (from_first, from_second),
                ((from_first, to_char), (to_char, from_second)),
            )
        })
        .collect();

    let mut freqs: HashMap<(char, char), u128> = HashMap::new();
    for wnd in original_template.chars().collect::<Vec<_>>().windows(2) {
        let entry = freqs.entry((wnd[0], wnd[1])).or_default();
        *entry += 1;
    }

    for _i in 0..40 {
        freqs = apply_optimized(&freqs, &rules);
    }

    let mut char_freqs: HashMap<char, u128> = HashMap::with_capacity(26);
    for ((cha, _chb), count) in freqs.into_iter() {
        let entry = char_freqs.entry(cha).or_default();
        *entry += count;
        // let entry = char_freqs.entry(chb).or_default();
        // *entry += count;
    }

    // Add 1 to the last char
    let entry = char_freqs
        .entry(original_template.chars().last().unwrap())
        .or_default();
    *entry += 1;

    let mut max = 0u128;
    let mut min = u128::MAX;
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

type Rules = HashMap<(char, char), ((char, char), (char, char))>;
fn apply_optimized(
    freqs: &HashMap<(char, char), u128>,
    rules: &Rules,
) -> HashMap<(char, char), u128> {
    let mut new_freqs = HashMap::with_capacity(freqs.len());
    for (pair, v) in freqs {
        let (a, b) = rules.get(pair).unwrap();
        let entry = new_freqs.entry(*a).or_default();
        *entry += v;
        let entry = new_freqs.entry(*b).or_default();
        *entry += v;
    }

    new_freqs
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
