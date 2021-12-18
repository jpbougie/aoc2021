use std::{fmt::Display, str::FromStr};

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/18.txt");
    let nums = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<SnNumber>>>()?;

    let mut base = nums.first().unwrap().clone();
    for n in nums.iter().skip(1) {
        base.add(n.clone());
    }

    println!("Part 1: {}", base.magnitude());

    let mut max = 0;
    for (i, a) in nums.iter().enumerate() {
        for (j, b) in nums.iter().enumerate() {
            if i != j {
                let mut base = a.clone();
                base.add(b.clone());

                let magnitude = base.magnitude();
                if magnitude > max {
                    max = magnitude;
                }

                let mut base = b.clone();
                base.add(a.clone());

                let magnitude = base.magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }

    println!("Part 2: {}", max);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Sn {
    Open,
    Close,
    Val(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnNumber {
    string: Vec<Sn>,
}

impl FromStr for SnNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ops = Vec::new();
        let mut chars = s.chars().peekable();
        let mut buf = String::new();
        while let Some(ch) = chars.next() {
            match ch {
                '[' => ops.push(Sn::Open),
                ']' => ops.push(Sn::Close),
                ',' => {}
                '0'..='9' => {
                    buf.push(ch);
                    if let Some(ch) = chars.peek() {
                        if !ch.is_digit(10) {
                            let val = buf.parse::<u32>()?;
                            ops.push(Sn::Val(val));
                            buf.clear();
                        }
                    }
                }
                _ => {
                    if !ch.is_whitespace() {
                        return Err(anyhow::format_err!("Could not understand {}", ch));
                    }
                }
            };
        }

        Ok(Self { string: ops })
    }
}

impl SnNumber {
    fn magnitude(&self) -> u64 {
        let mut it = self.string.iter().peekable();
        mag(&mut it)
    }

    fn add(&mut self, b: SnNumber) {
        let mut b = b;
        self.string.insert(0, Sn::Open);
        self.string.append(&mut b.string);
        self.string.push(Sn::Close);

        self.stabilize();
    }

    fn stabilize(&mut self) {
        while !(!self.explode() && !self.split()) {}
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for i in 0..self.string.len() {
            match self.string[i] {
                Sn::Open => depth += 1,
                Sn::Close => depth -= 1,
                Sn::Val(left) => {
                    if depth >= 5 {
                        let right = match self.string[i + 1] {
                            Sn::Val(v) => v,
                            _ => unreachable!("{:?}", self.string[i + 1]),
                        };

                        {
                            // find left neighbour
                            let mut leftsearch = i - 1;
                            while leftsearch > 0 {
                                match self.string[leftsearch] {
                                    Sn::Val(v) => {
                                        self.string[leftsearch] = Sn::Val(v + left);
                                        break;
                                    }
                                    _ => {
                                        leftsearch -= 1;
                                    }
                                }
                            }
                        }

                        {
                            // find right neighbour
                            let mut rightsearch = i + 2;
                            while rightsearch < self.string.len() {
                                match self.string[rightsearch] {
                                    Sn::Val(v) => {
                                        self.string[rightsearch] = Sn::Val(v + right);
                                        break;
                                    }
                                    _ => {
                                        rightsearch += 1;
                                    }
                                }
                            }
                        }

                        self.string.drain((i - 1)..=(i + 2));
                        self.string.insert(i - 1, Sn::Val(0));
                        return true;
                    }
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.string.len() {
            match self.string[i] {
                Sn::Val(v) if v >= 10 => {
                    let (left, right) = if v % 2 == 1 {
                        (v / 2, v / 2 + 1)
                    } else {
                        (v / 2, v / 2)
                    };

                    self.string[i] = Sn::Open;
                    self.string.insert(i + 1, Sn::Val(left));
                    self.string.insert(i + 2, Sn::Val(right));
                    self.string.insert(i + 3, Sn::Close);
                    return true;
                }
                _ => {}
            }
        }
        false
    }
}

fn mag(it: &mut std::iter::Peekable<std::slice::Iter<Sn>>) -> u64 {
    match it.peek() {
        Some(Sn::Open) => {
            it.next();
            let m = 3 * mag(it) + 2 * mag(it);
            it.next();
            m
        }
        Some(Sn::Val(v)) => {
            it.next();
            *v as u64
        }
        x => unreachable!("{:?}", x),
    }
}

impl Display for SnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = self.string.iter().peekable();
        while let Some(&item) = s.next() {
            match item {
                Sn::Val(v) => {
                    write!(f, "{}", v)?;
                    match s.peek() {
                        Some(Sn::Val(_)) | Some(Sn::Open) => {
                            write!(f, ",")?;
                        }
                        _ => {}
                    };
                }
                Sn::Open => write!(f, "[")?,
                Sn::Close => {
                    write!(f, "]")?;
                    match s.peek() {
                        Some(Sn::Val(_)) | Some(Sn::Open) => {
                            write!(f, ",")?;
                        }
                        _ => {}
                    };
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::SnNumber;

    #[test]
    fn add() {
        let mut in1 = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<SnNumber>().unwrap();
        let in2 = "[1, 1]".parse::<SnNumber>().unwrap();
        let out = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<SnNumber>()
            .unwrap();
        in1.add(in2);
        assert_eq!(out, in1);
        let mut in1 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<SnNumber>()
            .unwrap();
        let in2 = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            .parse::<SnNumber>()
            .unwrap();
        let out = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
            .parse::<SnNumber>()
            .unwrap();
        in1.add(in2);
        assert_eq!(out, in1);
    }

    #[test]
    fn magnitude() {
        assert_eq!(
            143,
            "[[1,2],[[3,4],5]]".parse::<SnNumber>().unwrap().magnitude()
        );
        assert_eq!(
            1384,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<SnNumber>()
                .unwrap()
                .magnitude()
        );
    }
}
