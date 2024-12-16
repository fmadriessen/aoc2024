use std::collections::HashMap;

use glam::IVec2;

pub mod part1;
pub mod part2;

fn print_grid(grid: &HashMap<IVec2, char>) {
    let width = grid.iter().map(|(p, _c)| p.x).max().unwrap();
    let height = grid.iter().map(|(p, _c)| p.y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            let p = IVec2::new(x, y);
            let c = *grid.get(&p).unwrap();

            print!("{c}");
        }
        println!();
    }
}

fn try_move(grid: &mut HashMap<IVec2, char>, position: IVec2, step: IVec2) -> bool {
    let next = position + step;
    let object = grid.get(&next).unwrap();

    if match object {
        'O' => try_move(grid, next, step),
        '#' => false,
        '.' => true,
        _ => unreachable!(),
    } {
        let a = grid.get_mut(&position).unwrap() as *mut char;
        let b = grid.get_mut(&next).unwrap() as *mut char;
        unsafe {
            std::ptr::swap(a, b);
        }
        true
    } else {
        false
    }
}

fn parse(input: &str) -> (HashMap<IVec2, char>, Vec<IVec2>) {
    let (grid, steps) = input.trim().split_once("\n\n").unwrap();
    let grid = parse_grid(grid);
    let steps = parse_commands(steps);

    (grid, steps)
}

fn parse_grid(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, c)| {
                let position = IVec2::new(x as i32, y as i32);
                (position, c)
            })
        })
        .collect()
}

fn parse_commands(input: &str) -> Vec<IVec2> {
    input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(IVec2::NEG_Y),
            'v' => Some(IVec2::Y),
            '>' => Some(IVec2::X),
            '<' => Some(IVec2::NEG_X),
            _ => None,
        })
        .collect()
}
