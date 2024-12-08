use glam::IVec2;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::LocatedSpan;

pub mod part1;
pub mod part2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Antenna {
    frequency: char,
    position: IVec2,
}

impl Antenna {
    pub fn new(frequency: char, position: IVec2) -> Antenna {
        Antenna {
            frequency,
            position,
        }
    }

    pub fn frequency(&self) -> char {
        self.frequency
    }

    pub fn position(&self) -> IVec2 {
        self.position
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

fn parse_antenna(input: Span) -> IResult<Span, Antenna> {
    let x = input.get_column() - 1;
    let y = input.location_line() - 1;

    let (input, frequency) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((
        input,
        Antenna::new(frequency, IVec2::new(x as i32, y as i32)),
    ))
}

pub fn parse(input: Span) -> IResult<Span, Vec<Antenna>> {
    let (input, antennas) = many1(preceded(
        take_till(|c: char| c.is_alphanum()),
        parse_antenna,
    ))(input)?;
    Ok((input, antennas))
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use super::*;

    #[test]
    fn test_parse() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let expected = vec![
            Antenna::new('0', IVec2::new(8, 1)),
            Antenna::new('0', IVec2::new(5, 2)),
            Antenna::new('0', IVec2::new(7, 3)),
            Antenna::new('0', IVec2::new(4, 4)),
            Antenna::new('A', IVec2::new(6, 5)),
            Antenna::new('A', IVec2::new(8, 8)),
            Antenna::new('A', IVec2::new(9, 9)),
        ];
        let (_input, output) = parse(Span::new(input)).unwrap();
        assert_eq!(output, expected);
    }
}
