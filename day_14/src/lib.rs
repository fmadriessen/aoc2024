use glam::IVec2;
use regex::Regex;

pub mod part1;
pub mod part2;

#[derive(Copy, Clone, Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse(input: &str) -> Vec<Robot> {
    let re_parameters =
        Regex::new(r"p=(?<px>\d+),(?<py>\d+)\s+v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let input: Vec<_> = input
        .trim()
        .split("\n")
        .map(|line| {
            let robot = re_parameters.captures(line).unwrap();
            let px = robot["px"].parse::<i32>().unwrap();
            let py = robot["py"].parse::<i32>().unwrap();
            let vx = robot["vx"].parse::<i32>().unwrap();
            let vy = robot["vy"].parse::<i32>().unwrap();
            Robot {
                position: IVec2::new(px, py),
                velocity: IVec2::new(vx, vy),
            }
        })
        .collect();

    input
}
