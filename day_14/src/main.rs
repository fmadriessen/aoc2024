const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
fn main() {
    let input = include_str!("../../inputs/day_14.txt");
    println!("part 1: {}", day_14::part1::process(input, WIDTH, HEIGHT));
    println!("part 2: {}", day_14::part2::process(input, WIDTH, HEIGHT));
}
