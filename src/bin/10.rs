use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/10.txt");
    let mut part1 = 0;
    let mut part2: Vec<u64> = Vec::new();
    for line in input.lines() {
        let (wrong, mut stack) = wrong_char(line);
        part1 += match wrong {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        };

        if stack.is_empty() {
            continue;
        }
        stack.reverse();
        part2.push(stack.into_iter().fold(0, |s, ch| match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        } + 5 * s));
    }

    part2.sort_unstable();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2[part2.len() / 2]);
    Ok(())
}

fn wrong_char(x: &str) -> (Option<char>, Vec<char>) {
    let mut stack = Vec::new();
    for ch in x.chars() {
        match ch {
            '(' | '[' | '<' | '{' => stack.push(ch),
            ')' => {
                if !expect_pop(&mut stack, '(') {
                    return (Some(ch), vec![]);
                }
            }
            ']' => {
                if !expect_pop(&mut stack, '[') {
                    return (Some(ch), vec![]);
                }
            }
            '}' => {
                if !expect_pop(&mut stack, '{') {
                    return (Some(ch), vec![]);
                }
            }
            '>' => {
                if !expect_pop(&mut stack, '<') {
                    return (Some(ch), vec![]);
                }
            }
            _ => {}
        };
    }

    (None, stack)
}

fn expect_pop(stack: &mut Vec<char>, ch: char) -> bool {
    if let Some(c) = stack.pop() {
        c == ch
    } else {
        true
    }
}
