use glam::IVec2;
use pathfinding::prelude::count_paths;

const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_X, IVec2::NEG_Y, IVec2::X, IVec2::Y];

pub fn process(input: &str) -> usize {
    let (_input, grid) = crate::parse(crate::Span::from(input)).unwrap();

    let result = grid
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(position, _)| {
            count_paths(
                *position,
                |position| {
                    DIRECTIONS
                        .iter()
                        .map(|direction| position + direction)
                        .filter(|successor| {
                            grid.get(successor)
                                .is_some_and(|height| grid.get(position).unwrap() + 1 == *height)
                        })
                        .collect::<Vec<_>>()
                },
                |position| grid.get(position).is_some_and(|height| *height == 9),
            )
        })
        .sum();

    result
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
        assert_eq!(81, process(input));
    }
}
