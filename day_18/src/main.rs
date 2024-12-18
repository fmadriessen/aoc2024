const SIZE: i32 = 70;
fn main() {
    let input = include_str!("../../inputs/day_18.txt");
    println!("part 1: {}", day_18::part1::process(input, SIZE, 1024));
    println!("part 2: {:?}", day_18::part2::process(input, SIZE));
}
