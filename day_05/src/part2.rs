use std::cmp::Ordering;

use crate::{is_correct_order, parse};

pub fn process(input: &str) -> usize {
    let (rules, mut updates) = parse(input);

    updates
        .iter_mut()
        .filter(|update| !is_correct_order(&rules, update))
        .map(|update| {
            update.sort_by(|a, b| {
                if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(123, process(input));
    }
}
