use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn process(input: &str, generations: usize) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .fold(HashMap::new(), |mut acc, number| {
            *acc.entry(number).or_insert(0) += 1;
            acc
        });

    for _ in 0..generations {
        let mut next_generation = HashMap::new();

        // VALUE           -> Rule
        // s(0)            -> s(1)
        // s(v: v % 2 = 0) -> x[v..v.len / 2] y[v..v.len/2]
        // s(v)            -> s(v * 2024)
        for (&number, &count) in &stones {
            if number == 0 {
                *next_generation.entry(1).or_default() += count;
            } else {
                let digits = number.ilog10() + 1;

                if digits % 2 == 0 {
                    let divisor = 10usize.pow(digits / 2);
                    *next_generation.entry(number % divisor).or_default() += count;
                    *next_generation.entry(number / divisor).or_default() += count;
                } else {
                    *next_generation.entry(number * 2024).or_default() += count;
                }
            }
        }

        stones = next_generation;
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "125 17";
        assert_eq!(55312, process(input, 25));
    }
}
