use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

fn binary_u32(input: &str) -> IResult<&str, u32> {
    let (rem, parsed) = many1(alt((value(1, tag("1")), value(0, tag("0")))))(input)?;

    Ok((
        rem,
        parsed
            .iter()
            .enumerate()
            .fold(0, |a, (i, d)| a | (d << (parsed.len() - 1 - i))),
    ))
}

const NUM_SIZE: usize = 12;

fn main() {
    let data = include_str!("day3.txt");
    let mut buckets = [0; NUM_SIZE];
    let parsed = many1(terminated(binary_u32, line_ending))(data).unwrap().1;
    println!("{:?}", parsed);

    for c in parsed.iter() {
        for i in 0..NUM_SIZE {
            if c & (1 << i) != 0 {
                buckets[i] += 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    for i in 0..NUM_SIZE {
        gamma |= (if buckets[i] > parsed.len() / 2 { 1 } else { 0 }) << i;
    }

    let epsilon = !gamma & 0b111111111111;

    println!("I have gamma = {}", gamma);
    println!("I have epsilon = {}", epsilon);
    println!("I have power consumption = {}", gamma * epsilon);
}
