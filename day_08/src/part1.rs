use itertools::Itertools;

use crate::{parse, Span};

pub fn process(input: &str) -> usize {
    let x_max = input.lines().next().unwrap().len() as i32;
    let y_max = input.lines().count() as i32;

    let (_input, mut antennas) = parse(Span::new(input)).unwrap();
    antennas.sort_by(|a, b| a.frequency.cmp(&b.frequency));
    let result = antennas
        .chunk_by(|a, b| a.frequency == b.frequency)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|antenna| {
                let diff = antenna[0].position - antenna[1].position;
                [antenna[0].position + diff, antenna[1].position - diff] // For antinodes, we do not care which
                                                                         // frequency caused them
            })
        })
        .filter(|node| (node.x >= 0 && node.x < x_max) && (node.y >= 0 && node.y < y_max))
        .unique()
        .count();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
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
        assert_eq!(process(input), 14);
    }
}
