use regex::Regex;

pub fn process(input: &str) -> usize {
    let find_instruction =
        Regex::new(r"(?<instruction>(?:(?:do|don't)\(\))|(?:mul\([0-9]{1,3},[0-9]{1,3}\)))")
            .unwrap();
    let find_parameters = Regex::new(r"(?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})").unwrap();

    let mut result: usize = 0;
    let mut enabled: bool = true;

    for (_, [instruction]) in find_instruction.captures_iter(input).map(|c| c.extract()) {
        match instruction {
            i if i.starts_with("mul") => {
                if enabled {
                    let params = find_parameters.captures(i).unwrap();
                    let x = &params["x"].parse::<usize>().unwrap();
                    let y = &params["y"].parse::<usize>().unwrap();

                    result += x * y;
                }
            }
            i if i.starts_with("don't") => {
                enabled = false;
            }
            i if i.starts_with("do") => {
                enabled = true;
            }
            _ => panic!(),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(process(input), 48);
    }
}
