use std::collections::HashMap;

use glam::IVec2;

pub mod part1;
pub mod part2;

pub fn create_grid_map(input: &str) -> HashMap<IVec2, char> {
    // { [0, 0] = 'M', [1, 0] = 'M', [1, 1] = 'S' ... }
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}
