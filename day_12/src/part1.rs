use crate::*;

pub fn process(input: &str) -> usize {
    let grid = parse(input);

    let mut visited: HashSet<IVec2> = HashSet::new();

    grid.iter().fold(0, |mut price, (position, _kind)| {
        if !visited.contains(position) {
            let (region, perimeter) = find_region(*position, &grid, &mut visited);
            price += region.len() * perimeter;
        }

        price
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(1930, process(input));
    }
}
