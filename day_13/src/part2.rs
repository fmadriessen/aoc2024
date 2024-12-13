use crate::*;

pub fn process(input: &str) -> i64 {
    let input = parse(input.trim());
    input
        .iter()
        .map(|machine| solve(machine, 10000000000000))
        .sum()
}
