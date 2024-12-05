pub mod part1;
pub mod part2;

pub fn convert_input_to_vecs(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();

        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_input_to_vecs() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(
            convert_input_to_vecs(input),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }
}
