use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

fn binary_u32(input: &str) -> IResult<&str, u32> {
    let (rem, parsed) = many1(alt((tag("1"), tag("0"))))(input)?;

    Ok((rem, u32::from_str_radix(&parsed.join(""), 2).unwrap()))
}

fn many1_binary_u32(input: &str) -> IResult<&str, Vec<u32>> {
    Ok(("", many1(terminated(binary_u32, line_ending))(input)?.1))
}

fn get_bit(i: usize, g: u32) -> bool {
    (g & 1 << i) > 0
}

fn set_bit(i: usize, g: u32, v: bool) -> u32 {
    if v {
        g | 1 << i
    } else {
        g & !(1 << i)
    }
}

const NUM_SIZE: usize = 12;

fn generate_buckets(data: &Vec<u32>) -> [usize; NUM_SIZE] {
    let mut buckets = [0; NUM_SIZE];

    for c in data.iter() {
        for i in 0..NUM_SIZE {
            if get_bit(i, *c) {
                buckets[i] += 1;
            }
        }
    }

    buckets
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = include_str!("day3.txt");
    let parsed = many1_binary_u32(data)?.1;
    let buckets = generate_buckets(&parsed);

    // Part 1
    let gamma = (0..NUM_SIZE).fold(0, |a, i| set_bit(i, a, buckets[i] > parsed.len() / 2));
    let epsilon = (0..NUM_SIZE).fold(0, |a, i| set_bit(i, a, buckets[i] < parsed.len() / 2));

    println!("I have gamma = {:0b}, {}", gamma, gamma);
    println!("I have epsilon = {:0b}, {}", epsilon, epsilon);
    println!("I have power consumption = {}", gamma * epsilon);

    // Part 2
    let mut oxygen = parsed.clone();
    let mut bit_index = NUM_SIZE;
    while oxygen.len() > 1 {
        bit_index -= 1;
        let buckets = generate_buckets(&oxygen);
        let comparand = buckets[bit_index] >= oxygen.len() - buckets[bit_index];

        oxygen = oxygen
            .into_iter()
            .filter(|d| get_bit(bit_index, *d) == comparand)
            .collect();
    }
    println!("I have oxygen generator rating {:?}", oxygen);

    let mut co2 = parsed.clone();
    let mut bit_index = NUM_SIZE;
    while co2.len() > 1 {
        bit_index -= 1;
        let buckets = generate_buckets(&co2);
        let comparand = buckets[bit_index] < co2.len() - buckets[bit_index];

        co2 = co2
            .into_iter()
            .filter(|d| get_bit(bit_index, *d) == comparand)
            .collect();
    }
    println!("I have CO2 scrubber rating {:?}", co2);
    println!("I have life support rating {:?}", co2[0] * oxygen[0]);

    Ok(())
}
