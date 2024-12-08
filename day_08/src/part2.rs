use glam::IVec2;
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
            chunk.iter().combinations(2).flat_map(|antennas| {
                let diff = antennas[0].position - antennas[1].position;

                let mut nodes: Vec<IVec2> = vec![];

                let mut k = 0;
                loop {
                    let node = antennas[0].position + k * diff;
                    if (node.x >= 0 && node.x < x_max) && (node.y >= 0 && node.y < y_max) {
                        nodes.push(node);
                        k += 1;
                    } else {
                        break;
                    }
                }

                let mut k = 0;
                loop {
                    let node = antennas[1].position - k * diff;
                    if (node.x >= 0 && node.x < x_max) && (node.y >= 0 && node.y < y_max) {
                        nodes.push(node);
                        k += 1;
                    } else {
                        break;
                    }
                }

                nodes
            })
        })
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
        assert_eq!(process(input), 34);
    }
}
