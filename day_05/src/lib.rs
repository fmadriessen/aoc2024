use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut input = input.lines();
    let rules = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut items = line.split('|');
            let a = items.next().unwrap().parse().unwrap();
            let b = items.next().unwrap().parse().unwrap();
            (a, b)
        })
        .fold(
            HashMap::default(),
            |mut acc: HashMap<usize, Vec<usize>>, (page, after)| {
                acc.entry(page)
                    .and_modify(|afters| afters.push(after))
                    .or_insert(vec![after]);
                acc
            },
        );

    let updates = input
        .map(|update| update.split(',').map(|u| u.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

/// Check whether a given update is in the correct order according to the given ordering rules
// INFO: Could also have used is_sorted_by similar to how we sorted the updates in part 2
pub fn is_correct_order(rules: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    for (i, page) in update.iter().enumerate() {
        let sub_update = &update[..i];
        if let Some(pages_after) = rules.get(page) {
            for p in sub_update.iter() {
                if pages_after.contains(p) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        let input = "75|29
75|53
75|47
75|61
75|13

75,47,61,53,29";
        let expected_rules = HashMap::from([(75, vec![29, 53, 47, 61, 13])]);
        let expected_updates = vec![vec![75, 47, 61, 53, 29]];
        assert_eq!(parse(input), (expected_rules, expected_updates));
    }

    #[test]
    fn test_is_correct_order() {
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
97,13,75,29,47";
        let (rules, updates) = parse(input);
        let output: Vec<bool> = updates
            .iter()
            .map(|u| is_correct_order(&rules, u))
            .collect();
        assert_eq!(output, [true, true, true, false, false, false]);
    }
}
