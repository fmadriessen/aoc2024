use crate::count_ways;
use std::collections::HashMap;

pub fn process(input: &str) -> usize {
    let (patterns, towels) = input.trim().split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let towels = towels.split("\n").collect::<Vec<_>>();

    let mut cache = HashMap::new();
    towels
        .into_iter()
        .filter(|&design| count_ways(design, &patterns, &mut cache) > 0)
        .count()
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
        assert_eq!(6, process(input));
    }
}
