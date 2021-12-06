use std::collections::HashMap;
use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::i16;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

pub fn gcd(a: i16, b: i16) -> i16 {
    // Terminal cases
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let a_is_even = a % 2 == 0;
    let b_is_even = b % 2 == 0;

    match (a_is_even, b_is_even) {
        (true, true) => gcd(a / 2, b / 2) * 2,
        (true, false) => gcd(a / 2, b),
        (false, true) => gcd(a, b / 2),
        (false, false) => {
            if a > b {
                gcd((a - b) / 2, b)
            } else {
                gcd((b - a) / 2, a)
            }
        }
    }
}

impl Line {
    fn plot(&self, map: &mut HashMap<Point, u16>) {
        // Calculate the slope of this line
        let (mut rise, mut run) = ((self.b.y - self.a.y), (self.b.x - self.a.x));

        let gcd = gcd(rise.abs(), run.abs());
        rise = rise / gcd;
        run = run / gcd;

        let mut cur = self.a;
        *map.entry(cur).or_insert(0) += 1;

        while cur != self.b {
            cur.x += run;
            cur.y += rise;
            *map.entry(cur).or_insert(0) += 1;
        }
    }

    fn straight(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let (rem, (x, y)) = separated_pair(i16, char(','), i16)(input)?;

    Ok((rem, Point { x, y }))
}

fn line(input: &str) -> IResult<&str, Line> {
    let (rem, (a, b)) = separated_pair(point, tag(" -> "), point)(input)?;
    Ok((rem, Line { a, b }))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut straight_coords = HashMap::new();
    let mut coords = HashMap::new();

    let lines = many1(terminated(line, line_ending))(include_str!("input.txt"))?.1;
    println!("I have {} lines", lines.len());
    for l in lines {
        l.plot(&mut coords);
        if l.straight() {
            l.plot(&mut straight_coords);
        }
    }

    println!(
        "I have {} overlaps (straight only) and {} overlaps (all lines)",
        straight_coords.iter().filter(|(_, &v)| v >= 2).count(),
        coords.iter().filter(|(_, &v)| v >= 2).count()
    );

    Ok(())
}
