use crate::{find_path_at_t, parse, Pos};

pub fn process(input: &str, size: i32) -> Pos {
    let input = parse(input);

    let mut low = 0_usize;
    let mut high = input.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if find_path_at_t(&input[0..mid], size, mid).is_none() {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    input[low - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        assert_eq!(Pos(6, 1), process(input, 6));
    }
}
