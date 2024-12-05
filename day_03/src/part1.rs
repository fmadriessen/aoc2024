use regex::Regex;

pub fn process(input: &str) -> usize {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [x, y])| {
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            x * y
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process(input), 161);
    }
}
