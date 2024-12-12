use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

pub mod part1;
pub mod part2;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_Y, IVec2::NEG_X, IVec2::Y];

fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect()
}

fn find_neighbours(seed: IVec2, grid: &HashMap<IVec2, char>, kind: char) -> Vec<IVec2> {
    let mut neighbours = Vec::new();

    for direction in DIRECTIONS {
        let neighbour = seed + direction;

        if grid.get(&neighbour).is_some_and(|c| *c == kind) {
            neighbours.push(neighbour);
        }
    }

    neighbours
}

pub fn find_region(
    seed: IVec2,
    grid: &HashMap<IVec2, char>,
    visited: &mut HashSet<IVec2>,
) -> (HashSet<IVec2>, usize) {
    let mut queue = VecDeque::new();
    let kind = grid.get(&seed).unwrap();

    let mut region: HashSet<IVec2> = HashSet::default();
    let mut perimeter = 0;

    visited.insert(seed);
    queue.push_back(seed);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        region.insert(current);

        let neighbours = find_neighbours(current, grid, *kind);
        perimeter += 4 - neighbours.len();

        for neighbour in neighbours {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                queue.push_back(neighbour);
            }
        }
    }

    (region, perimeter)
}
