use itertools::Itertools;

use crate::*;
pub fn process(input: &str) -> usize {
    let grid = parse(input);

    let mut visited: HashSet<IVec2> = HashSet::new();
    grid.iter().fold(0, |mut price, (position, kind)| {
        if !visited.contains(position) {
            let (region, _perimeter) = find_region(*position, &grid, &mut visited);
            let sides = count_corners(&region, &grid, kind);
            price += region.len() * sides;
        }
        price
    })
}

fn count_corners(region: &HashSet<IVec2>, grid: &HashMap<IVec2, char>, kind: &char) -> usize {
    let mut count = 0;

    for position in region {
        for (d1, d2) in DIRECTIONS.iter().circular_tuple_windows() {
            let next1 = position + d1;
            let next2 = position + d2;
            let next_diag = position + d1 + d2;

            let test1 = grid.get(&next1).is_some_and(|c| c == kind);
            let test2 = grid.get(&next2).is_some_and(|c| c == kind);

            if (test1 && test2 && grid.get(&next_diag).is_some_and(|c| c != kind))
                || (!test1 && !test2)
            {
                count += 1;
            }
        }
    }

    count
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
        assert_eq!(1206, process(input));
    }
}
