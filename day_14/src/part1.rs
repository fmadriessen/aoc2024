use glam::IVec2;

use crate::{parse, Robot};

const TIME: i32 = 100;
pub fn process(input: &str, width: i32, height: i32) -> usize {
    let mut robots = parse(input);

    let robots: Vec<Robot> = robots
        .iter_mut()
        .map(|robot| {
            let correction = TIME * IVec2::new(width, height);
            let end = robot.position + TIME * robot.velocity + correction;
            let end = IVec2::new(end.x % width, end.y % height);
            robot.position = end;
            *robot
        })
        .collect();

    let split_x = width / 2;
    let split_y = height / 2;

    let q1 = robots
        .iter()
        .filter(|robot| robot.position.x > split_x && robot.position.y > split_y)
        .count();
    let q2 = robots
        .iter()
        .filter(|robot| robot.position.x < split_x && robot.position.y > split_y)
        .count();
    let q3 = robots
        .iter()
        .filter(|robot| robot.position.x < split_x && robot.position.y < split_y)
        .count();
    let q4 = robots
        .iter()
        .filter(|robot| robot.position.x > split_x && robot.position.y < split_y)
        .count();

    q1 * q2 * q3 * q4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(12, process(input, 11, 7));
    }
}
