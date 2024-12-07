use crate::{is_tractable, parse};

pub fn process(input: &str) -> u64 {
    let (_input, equations) = parse(input).unwrap();

    let result = equations
        .iter()
        .filter_map(|(result, numbers)| {
            if is_tractable(*result, numbers, true) {
                return Some(result);
            }

            None
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, process(input));
    }
}
