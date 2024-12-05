use itertools::Itertools;

pub mod part1;
pub mod part2;

type Report = Vec<isize>;

enum Direction {
    Increasing,
    Decreasing,
}

pub fn convert_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_safe(report: &Report) -> bool {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        let bounded = (1..=3).contains(&diff.abs());

        if !bounded {
            return false;
        }

        match diff.signum() {
            -1 => match direction {
                None => {
                    direction = Some(Direction::Increasing);
                }
                Some(Direction::Decreasing) => {
                    return false;
                }
                Some(Direction::Increasing) => {}
            },
            1 => match direction {
                None => {
                    direction = Some(Direction::Decreasing);
                }
                Some(Direction::Decreasing) => {}
                Some(Direction::Increasing) => {
                    return false;
                }
            },
            _ => panic!(),
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_input() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(
            convert_input(input),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }
}
