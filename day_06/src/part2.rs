use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

use crate::{parse, Direction, Span};

pub fn process(input: &str) -> usize {
    let (_input, ((mut position, _), obstacles)) = parse(Span::new(input)).unwrap();
    let origin = position;
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
    visited.remove(&origin);

    visited
        .iter()
        .filter(|added_wall| {
            let mut position = origin;
            let mut direction = Direction::North;

            let mut new_visited = HashSet::from([(position, direction.clone())]);

            while (x_min..=x_max).contains(&position.x) && (y_min..=y_max).contains(&position.y) {
                let next = position + direction.to_ivec2();

                if obstacles.contains_key(&next) || next == **added_wall {
                    direction = direction.turn_right();
                    continue;
                } else if new_visited.contains(&(next, direction.clone())) {
                    return true;
                } else {
                    position = next;
                    new_visited.insert((position, direction.clone()));
                }
            }

            false
        })
        .count()
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
        assert_eq!(6, process(input));
    }
}
