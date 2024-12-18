use std::collections::HashSet;

use pathfinding::prelude::bfs;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(i32, i32);

impl Pos {
    fn successors(&self) -> Vec<Self> {
        vec![
            *self + Pos(0, 1),
            *self + Pos(0, -1),
            *self + Pos(1, 0),
            *self + Pos(-1, 0),
        ]
    }
}
impl std::ops::Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

fn find_path_at_t(input: &[Pos], size: i32, time: usize) -> Option<Vec<Pos>> {
    let walls = input[0..time].iter().collect::<HashSet<_>>();

    bfs(
        &Pos(0, 0),
        |p| {
            p.successors().into_iter().filter(|s| {
                !walls.contains(&s) && (0..=size).contains(&s.0) && (0..=size).contains(&s.1)
            })
        },
        |p| p == &Pos(size, size),
    )
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Pos(x, y)
        })
        .collect()
}
