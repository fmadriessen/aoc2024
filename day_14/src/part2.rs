use std::collections::HashSet;

use crate::parse;
use glam::IVec2;

pub fn process(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse(input);

    let mut time = 0;
    let correction = IVec2::new(width, height);
    'simulation: loop {
        time += 1;
        let mut visited: HashSet<IVec2> = HashSet::new();

        for robot in &mut robots {
            robot.position += robot.velocity + correction;
            robot.position.x %= width;
            robot.position.y %= height;

            if !visited.contains(&robot.position) {
                visited.insert(robot.position);
            }
        }

        if visited.len() == robots.len() {
            for y in 0..height {
                for x in 0..width {
                    if visited.contains(&IVec2::new(x, y)) {
                        print!(".");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            break 'simulation;
        }
    }

    time
}
