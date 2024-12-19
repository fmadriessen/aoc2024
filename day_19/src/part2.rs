use std::collections::HashMap;

use crate::count_ways;

pub fn process(input: &str) -> usize {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let towels = towels.split("\n").collect::<Vec<_>>();

    let mut cache = HashMap::new();
    towels
        .into_iter()
        .map(|design| count_ways(design, &patterns, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(16, process(input));
    }
}
