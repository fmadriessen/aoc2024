use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub mod part1;
pub mod part2;

fn parse(input: &str) -> Map {
    let items = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<IVec2, char>>();

    let start = *items.iter().find(|(_p, c)| c == &&'S').unwrap().0;
    let end = *items.iter().find(|(_p, c)| c == &&'E').unwrap().0;
    let walls = items
        .into_iter()
        .filter_map(|(p, c)| match c {
            '#' => Some(p),
            _ => None,
        })
        .collect::<HashSet<IVec2>>();

    Map { start, end, walls }
}

struct Map {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}
