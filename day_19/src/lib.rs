use std::collections::HashMap;

pub mod part1;
pub mod part2;

fn count_ways<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let mut count = 0;
    for &pattern in patterns {
        if let Some(remaining) = design.strip_prefix(pattern) {
            count += count_ways(remaining, patterns, cache);
        }
    }

    cache.insert(design, count);
    count
}
