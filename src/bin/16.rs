use anyhow::Result;
use nom::{
    bits,
    bits::complete::tag,
    bits::complete::take,
    branch::alt,
    multi::{count, many1},
    sequence::tuple,
    IResult,
};

fn main() -> Result<()> {
    let input = include_str!("../../inputs/16.txt");
    let bytes = hex::decode(input).unwrap();

    if let Ok((_x, program)) = parse(&bytes) {
        println!("{:?}", program);
        println!("{:?}", program.version_sum());
        println!("{:?}", program.eval());
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Program {
    version: u8,
    expr: Expr,
    programs: Vec<Program>,
}

impl Program {
    fn version_sum(&self) -> u32 {
        (self.version as u32)
            + self
                .programs
                .iter()
                .map(|program| program.version_sum())
                .sum::<u32>()
    }

    fn eval(&self) -> u64 {
        match self.expr {
            Expr::Value(x) => x as u64,
            Expr::Operator(OpType::Sum) => self
                .programs
                .iter()
                .map(|program| program.eval())
                .sum::<u64>(),
            Expr::Operator(OpType::Product) => self
                .programs
                .iter()
                .map(|program| program.eval())
                .product::<u64>(),
            Expr::Operator(OpType::Min) => self
                .programs
                .iter()
                .map(|program| program.eval())
                .min()
                .unwrap(),
            Expr::Operator(OpType::Max) => self
                .programs
                .iter()
                .map(|program| program.eval())
                .max()
                .unwrap(),
            Expr::Operator(OpType::Gt) => {
                if self.programs[0].eval() > self.programs[1].eval() {
                    1
                } else {
                    0
                }
            }
            Expr::Operator(OpType::Lt) => {
                if self.programs[0].eval() < self.programs[1].eval() {
                    1
                } else {
                    0
                }
            }
            Expr::Operator(OpType::Eq) => {
                if self.programs[0].eval() == self.programs[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Value(u64),
    Operator(OpType),
}

#[derive(Debug, Clone)]
enum OpType {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}
impl From<u8> for OpType {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &[u8]) -> IResult<&[u8], Program> {
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(op)(input)
}

fn op(input: (&[u8], usize)) -> IResult<(&[u8], usize), Program> {
    let (input, (version, (expr, programs))) = tuple((version, parse_op))(input)?;

    Ok((
        input,
        Program {
            version,
            expr,
            programs,
        },
    ))
}

type Res<'a, T> = IResult<(&'a [u8], usize), T>;

fn version(input: (&[u8], usize)) -> Res<u8> {
    take(3usize)(input)
}

fn parse_op(input: (&[u8], usize)) -> Res<(Expr, Vec<Program>)> {
    alt((value, operator))(input)
}

fn value(input: (&[u8], usize)) -> Res<(Expr, Vec<Program>)> {
    let (input, _tag): ((&[u8], usize), u8) = tag(4u8, 3usize)(input)?;
    let mut val = 0u64;
    let mut input = input;
    while let Ok((i, _1)) = tag::<_, _, _, nom::error::Error<(&[u8], usize)>>(1u8, 1usize)(input) {
        let (i, bytes): (_, u8) = take(4u8)(i)?;
        input = i;
        val <<= 4;
        val += bytes as u64;
    }
    let (input, _tag): ((&[u8], usize), u8) = tag(0u8, 1usize)(input)?;
    let (input, bytes): (_, u8) = take(4u8)(input)?;
    val <<= 4;
    val += bytes as u64;

    Ok((input, (Expr::Value(val), Vec::new())))
}

fn operator(input: (&[u8], usize)) -> Res<(Expr, Vec<Program>)> {
    let (input, expr): ((&[u8], usize), u8) = take(3usize)(input)?;
    let (input, mode): (_, u8) = take(1usize)(input)?;
    if mode == 0 {
        let (input, bit_length): ((&[u8], usize), usize) = take(15usize)(input)?;
        let mut sub_input = Vec::new();
        let mut bit_length = bit_length;
        let mut input = input;
        while bit_length > 8 {
            let (i, byte): ((&[u8], usize), u8) = take(8usize)(input)?;
            input = i;
            sub_input.push(byte);
            bit_length -= 8;
        }

        let (input, leftover): (_, u8) = take(bit_length)(input)?;
        sub_input.push(leftover << (8 - bit_length));

        let (_, programs) = bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(many1(op))(
            &sub_input,
        )
        .map_err(|e: nom::Err<nom::error::Error<&[u8]>>| {
            e.map(|_e| nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        })?;

        Ok((input, (Expr::Operator(expr.into()), programs)))
    } else {
        let (input, program_len): ((&[u8], usize), usize) = take(11usize)(input)?;
        let (input, programs) = count(op, program_len)(input)?;
        Ok((input, (Expr::Operator(expr.into()), programs)))
    }
}
