use crate::{parse, Direction, Span};
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashSet;

pub fn process(input: &str) -> usize {
    let (_input, ((mut position, _), obstacles)) = parse(Span::new(input)).unwrap();
    let mut direction = Direction::North;

    let (x_min, x_max) = obstacles
        .keys()
        .map(|p| p.x)
        .minmax()
        .into_option()
        .unwrap();
    let (y_min, y_max) = obstacles
        .keys()
        .map(|p| p.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut visited: HashSet<IVec2> = HashSet::from([position]);
    while (x_min..=x_max).contains(&position.x) && (y_min..=y_max).contains(&position.y) {
        let next = position + direction.to_ivec2();

        if obstacles.contains_key(&next) {
            direction = direction.turn_right();
        } else {
            position = next;
            visited.insert(position);
        }
    }

    visited.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(41, process(input));
    }
}
