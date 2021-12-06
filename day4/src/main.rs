use std::error::Error;

use nom::character::complete::char;
use nom::character::complete::multispace1;
use nom::character::complete::u16;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::IResult;
use nom::{multi::separated_list1, sequence::terminated};
struct Board {
    data: Vec<(bool, u16)>,
    last_chosen: Option<u16>,
}

impl Board {
    pub fn wins(&self) -> bool {
        // Rows
        self.data[..].chunks(5).any(|r| r.iter().all(|i| i.0))
        // Columns
        || (0..5).map(|x| (0..5).map(move |y| self.data[y + y*x])).any(|mut c| c.all(|(b, _)| b))
    }

    pub fn score(&self) -> u16 {
        self.last_chosen.unwrap() * self.sum_unmarked()
    }

    pub fn sum_unmarked(&self) -> u16 {
        self.data
            .iter()
            .filter(|(chosen, _)| !chosen)
            .map(|(_, value)| value)
            .fold(0, |acc, value| acc + *value)
    }

    pub fn apply(&mut self, choice: u16) {
        for (chosen, val) in self.data.iter_mut() {
            if *val == choice {
                *chosen = true;
                self.last_chosen = Some(choice);
            }
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u16>, Vec<Board>)> {
    // Parse the number list.
    let (rem, nums) = terminated(separated_list1(char(','), u16), multispace1)(input)?;
    let (rem, boards) = many1(map(many_m_n(25, 25, terminated(u16, multispace1)), |b| {
        Board {
            data: b.iter().map(|i| (false, *i)).collect(),
            last_chosen: None,
        }
    }))(rem)?;

    Ok((rem, (nums, boards)))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut numbers, mut boards) = parse_input(include_str!("../input.txt"))?.1;

    while !boards.iter().any(|b| b.wins()) {
        let n = numbers.remove(0);
        for b in boards.iter_mut() {
            b.apply(n);
        }
    }

    if let Some((index, board)) = boards.iter().enumerate().find(|(_, board)| board.wins()) {
        println!(
            "I have a winner at index {} with score {} (last chosen is {})",
            index,
            board.score(),
            board.last_chosen.unwrap()
        );
    }

    Ok(())
}
