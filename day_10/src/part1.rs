use std::collections::{HashMap, HashSet};

use glam::IVec2;
const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_X, IVec2::NEG_Y, IVec2::X, IVec2::Y];

pub fn process(input: &str) -> usize {
    let (_input, grid) = crate::parse(crate::Span::from(input)).unwrap();

    let starting_positions: HashSet<IVec2> = grid
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(position, _)| *position)
        .collect();

    let mut trails: HashMap<IVec2, usize> = HashMap::new();

    for position in starting_positions.iter() {
        let mut visited: HashSet<IVec2> = HashSet::new();
        let mut successors: HashSet<IVec2> = HashSet::from([*position]);

        while !successors.is_empty() {
            let next_generation = successors
                .iter()
                .flat_map(|start| {
                    DIRECTIONS
                        .iter()
                        .map(move |direction| (direction + start, start))
                })
                .filter(|(successor, start)| {
                    !successors.contains(successor)
                        && grid
                            .get(successor)
                            .is_some_and(|height| *height == grid.get(start).unwrap() + 1)
                })
                .map(|(successor, _)| successor)
                .collect::<HashSet<IVec2>>();

            for successor in next_generation.iter() {
                visited.insert(*successor);
            }

            successors = next_generation;
        }

        trails.insert(
            *position,
            visited
                .iter()
                .filter(|position| *grid.get(position).unwrap() == 9)
                .count(),
        );
    }

    trails.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!(36, process(input));
    }
}
