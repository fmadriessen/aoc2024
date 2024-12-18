use crate::{find_path_at_t, parse};

pub fn process(input: &str, size: i32, time: usize) -> usize {
    let input = parse(input);
    let path = find_path_at_t(&input, size, time);

    path.unwrap().len() - 1 // Off by one, since we include the start position in the path
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
        assert_eq!(22, process(input, 6, 12));
    }
}
