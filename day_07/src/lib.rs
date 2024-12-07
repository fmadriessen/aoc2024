use std::num::IntErrorKind;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub mod part1;
pub mod part2;

pub fn is_tractable(r: u64, n: &[u64], concatenate: bool) -> bool {
    // If an equation is given as r = [a1, a2, ... an]
    // then
    //    last operator can only be concatenation if r[n] == [an]
    //    last operator can only be a multiplication if r % an == 0
    //    last operator can only be a sum if r - an >= 0, i.e. if r >= an
    // we then know what should be the lhs of the operator,
    //    x = 10^{width an} * a{n-1} + an;
    //    x: an * x = r -> x = r / an
    //    x: an + x = r -> x = r - an
    // at which point we can recurse on said remainder, is_tractable(x, [a1 ... a{n - 1}])
    //
    // Note: It is possible poth paths hold true at a given position, so we should probably keep
    // track of both

    let length = n.len();
    if length == 1 {
        return r == n[0];
    }

    let x = n.last().unwrap();

    if concatenate {
        let x_str = x.to_string();
        let r_str = r.to_string();

        if r_str.ends_with(&x_str) {
            let y_str = &r_str[..r_str.len() - x_str.len()];
            match y_str.parse::<u64>() {
                Ok(y) => {
                    if is_tractable(y, &n[..length - 1], concatenate) {
                        return true;
                    }
                }
                Err(err) => {
                    if *err.kind() == IntErrorKind::Empty {
                        let tractable = is_tractable(1, &n[..length - 1], false);
                        return tractable;
                    }
                }
            }
        }
    }

    if let Some(y) = divide(r, *x) {
        if is_tractable(y, &n[..length - 1], concatenate) {
            return true;
        };
    }

    if let Some(y) = subtract(r, *x) {
        return is_tractable(y, &n[..length - 1], concatenate);
    }

    false
}

fn subtract(r: u64, x: u64) -> Option<u64> {
    if r > x {
        return Some(r - x);
    }

    None
}

fn divide(r: u64, x: u64) -> Option<u64> {
    if r >= x && r % x == 0 {
        return Some(r / x);
    }

    None
}

pub fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
        let expected = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (161011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        let (_input, result) = parse(input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_is_multiplicable() {
        assert_eq!(divide(190, 10), Some(19));
        assert_eq!(divide(190, 19), Some(10));
        assert_eq!(divide(156, 15), None);
    }

    #[test]
    fn test_is_summable() {
        assert_eq!(subtract(11, 6), Some(5));
        assert_eq!(subtract(6, 11), None);
    }

    #[test]
    fn test_is_tractable_without_concatenation() {
        assert_eq!(is_tractable(190, &vec![10, 19], false), true);
        assert_eq!(is_tractable(83, &vec![17, 5], false), false);
        assert_eq!(is_tractable(156, &vec![15, 6], false), false);
        assert_eq!(is_tractable(7290, &vec![6, 8, 6, 15], false), false);
    }

    #[test]
    fn test_is_tractable_with_concatenation() {
        assert_eq!(is_tractable(190, &vec![10, 19], true), true);
        assert_eq!(is_tractable(83, &vec![17, 5], true), false);
        assert_eq!(is_tractable(156, &vec![15, 6], true), true);
        assert_eq!(is_tractable(7290, &vec![6, 8, 6, 15], true), true);
    }
}
