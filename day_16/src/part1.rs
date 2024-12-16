use glam::IVec2;
use pathfinding::prelude::dijkstra;

use crate::{parse, Map};

pub fn process(input: &str) -> i32 {
    const COST_TURN: i32 = 1000;
    const COST_FORWARD: i32 = 1;

    let Map { start, end, walls } = parse(input);

    let (_path, cost) = dijkstra(
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
        |&(position, _)| position == end,
    )
    .expect("No path found");

    cost
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
        7036
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
        11048
    )]
    fn test_process(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, process(input));
    }
}
