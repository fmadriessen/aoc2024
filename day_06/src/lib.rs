use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

pub mod part1;
pub mod part2;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::Y,
            Direction::West => IVec2::NEG_X,
        }
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

type Item = (IVec2, char);

pub fn parse(input: Span) -> IResult<Span, (Item, HashMap<IVec2, char>)> {
    let (input, items) = separated_list1(line_ending, many1(token))(input)?;

    let guard = items
        .iter()
        .flatten()
        .find(|(_, c)| c == &'^')
        .cloned()
        .expect("Could not find player");
    let obstacles = items
        .into_iter()
        .flatten()
        .filter(|(_, c)| c == &'#')
        .collect();

    Ok((input, (guard, obstacles)))
}

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line() - 1;
    let x = input.get_column() - 1;

    let (input, token) = one_of(".#^")(input)?;

    Ok((input, (IVec2::new(x as i32, y as i32), token)))
}
