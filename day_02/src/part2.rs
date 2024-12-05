use crate::{convert_input, is_safe};

pub fn process(input: &str) -> usize {
    let reports = convert_input(input);
    reports
        .iter()
        .filter(|report| {
            if !is_safe(report) {
                for i in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(i);
                    if is_safe(&new_report) {
                        return true;
                    };
                }
                return false;
            } else {
                return true;
            }
        })
        .count()
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
        assert_eq!(process(input), 4);
    }
}
