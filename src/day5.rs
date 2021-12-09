

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<&str> {
    input.lines().flat_map(|s| s.split(" -> ")).collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<&str>) -> u32 {
    println!("{:?}", input);
    0
}