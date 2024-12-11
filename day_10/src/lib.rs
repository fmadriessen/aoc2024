use nom::character::complete::satisfy;
use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;

pub mod part1;
pub mod part2;

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, positions) = separated_list1(line_ending, many1(numeric_position))(input)?;

    let grid: HashMap<IVec2, u32> = positions.iter().flatten().copied().collect();
    Ok((input, grid))
}

fn numeric_position(input: Span) -> IResult<Span, (IVec2, u32)> {
    let x = input.get_column() - 1;
    let y = input.location_line() - 1;

    let (input, c) = satisfy(|c| c.is_numeric())(input)?;
    Ok((
        input,
        (IVec2::new(x as i32, y as i32), c.to_digit(10).unwrap()),
    ))
}
