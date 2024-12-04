use std::iter::zip;

use crate::convert_input_to_vecs;

pub fn process(input: &str) -> usize {
    let (mut left, mut right) = convert_input_to_vecs(input);

    left.sort();
    right.sort();

    let result: usize = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, process(input));
    }
}
