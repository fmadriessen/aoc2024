use crate::create_grid_map;
use glam::IVec2;

const DIRECTIONS: [[IVec2; 2]; 4] = [
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
];

pub fn process(input: &str) -> usize {
    let positions = create_grid_map(input);

    let search_for = ['M', 'S'];
    let result = positions
        .iter()
        .filter(|(_position, value)| **value == 'A')
        .filter(|(position, _value)| {
            let count_xmas = DIRECTIONS
                .iter()
                .map(|dir_vectors| {
                    dir_vectors
                        .iter()
                        .map(|offset| positions.get(&(*position + offset)))
                        .enumerate()
                        .all(|(index, value)| search_for.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                == 2;
            count_xmas
        })
        .count();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!(9, process(input));
    }
}
