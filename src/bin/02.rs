use std::{
    io::{self, ErrorKind, Read},
    str::FromStr,
};

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let mut f = std::fs::File::open("inputs/02.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let ops: Vec<Instruction> = s
        .lines()
        .map(FromStr::from_str)
        .collect::<anyhow::Result<_>>()?;

    let mut sub = Sub { pos: 0, depth: 0 };

    for op in ops.iter() {
        match *op {
            Instruction::Down(d) => sub.depth += d,
            Instruction::Up(d) => sub.depth -= d,
            Instruction::Forward(x) => sub.pos += x,
        }
    }

    println!("Part 1: {}", sub.score());

    let mut sub2: Sub2 = Default::default();
    for op in ops.iter() {
        match *op {
            Instruction::Down(d) => sub2.aim += d,
            Instruction::Up(d) => sub2.aim -= d,
            Instruction::Forward(x) => {
                sub2.pos += x;
                sub2.depth += x * sub2.aim
            }
        }
    }

    println!("Part 2: {}", sub2.score());

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        if let Some(instr) = parts.next() {
            match instr {
                "forward" => Ok(Self::Forward(
                    parts.next().ok_or(anyhow!("missing operand"))?.parse()?,
                )),
                "up" => Ok(Self::Up(
                    parts.next().ok_or(anyhow!("missing operand"))?.parse()?,
                )),
                "down" => Ok(Self::Down(
                    parts.next().ok_or(anyhow!("missing operand"))?.parse()?,
                )),
                x => Err(anyhow!("not a valid instruction: {}", x)),
            }
        } else {
            Err(anyhow!("not a valid instruction"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sub {
    pos: i64,
    depth: i64,
}

impl Sub {
    pub fn score(&self) -> i64 {
        self.pos * self.depth
    }
}

#[derive(Default, Debug)]
pub struct Sub2 {
    pos: i64,
    depth: i64,
    aim: i64,
}

impl Sub2 {
    pub fn score(&self) -> i64 {
        self.pos * self.depth
    }
}
