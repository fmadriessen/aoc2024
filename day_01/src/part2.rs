use crate::convert_input_to_vecs;

pub fn process(input: &str) -> usize {
    let (left, right) = convert_input_to_vecs(input);

    let result: usize = left
        .iter()
        .map(|num| num * right.iter().filter(|r| &num == r).count())
        .sum();

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
        assert_eq!(process(input), 31);
    }
}
