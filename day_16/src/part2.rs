use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::astar_bag;

use crate::{parse, Map};
pub fn process(input: &str) -> i32 {
    const COST_TURN: i32 = 1000;
    const COST_FORWARD: i32 = 1;

    let Map { start, end, walls } = parse(input);

    // Find the nodes that are part of *any* best path

    let (paths, _cost) = astar_bag(
        &(start, IVec2::X),
        |(position, direction)| {
            let next_position = position + direction;
            if walls.contains(&next_position) {
                vec![
                    ((*position, direction.perp()), COST_TURN),
                    ((*position, -direction.perp()), COST_TURN),
                ]
            } else {
                vec![
                    ((next_position, *direction), COST_FORWARD),
                    ((*position, direction.perp()), COST_TURN),
                    ((*position, -direction.perp()), COST_TURN),
                ]
            }
        },
        |_| 0,
        |&(position, _)| position == end,
    )
    .expect("No path found");

    let visited_positions = paths
        .flat_map(|path| path.into_iter().map(|(position, _direction)| position))
        .collect::<HashSet<IVec2>>();

    visited_positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        45
    )]
    #[case(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        64
    )]
    fn test_process(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, process(input));
    }
}
