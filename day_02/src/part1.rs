use super::*;

// Consider using windows
pub fn process(input: &str) -> usize {
    let reports = crate::convert_input(input);
    reports.iter().filter(|report| is_safe(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process(input), 2);
    }
}
