use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::{line_ending, u32, char};
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom::combinator::value;
use nom::multi::many1;

#[derive(Clone)]
enum SubDir {
    Forward,
    Up,
    Down,
}

#[derive(Clone)]
struct SubCmd(SubDir, u32);

struct SubState {
    h: u32,
    d: u32,
    aim: u32,
}

impl SubState {
    pub fn execute(&mut self, cmd: &SubCmd) {
        match cmd {
            SubCmd(SubDir::Forward, dist) => {
                self.h += dist;
                self.d += self.aim * dist;
            }
            SubCmd(SubDir::Up, dist) => self.aim -= dist,
            SubCmd(SubDir::Down, dist) => self.aim += dist,
        }
    }

    pub fn result(&self) -> u32 {
        self.h * self.d
    }
}

fn sub_cmd(input: &str) -> IResult<&str, SubCmd> {
    let result = separated_pair(
        alt((
            value(SubDir::Up, tag("up")),
            value(SubDir::Down, tag("down")),
            value(SubDir::Forward, tag("forward")),
        )),
        char(' '),
        u32,
    )(input)?;

    Ok((result.0, SubCmd(result.1.0, result.1.1)))
}

fn main() {
    let data = include_str!("day2.txt");
    let mut s = SubState { aim: 0, d: 0, h: 0 };

    for c in many1(terminated(sub_cmd, line_ending))(data).unwrap().1 {
        s.execute(&c);
    }

    println!("Result: {}", s.result());
}
