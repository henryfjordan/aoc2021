#[aoc_generator(day1)]
fn generator(input: &str) -> Vec<u32> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<u32>) -> u32 {
    let mut increment_counter: u32 = 0;
    let mut last_value: u32 = 999999999;

    for i in input.iter() {
        if i > &last_value {
            increment_counter = increment_counter + 1;
        }
        last_value = *i;
    }

    increment_counter
}

#[aoc(day1, part2)]
fn part2(input: &Vec<u32>) -> u32 {
    let mut increment_counter: u32 = 0;
    let mut last_value: u32 = 999999999;

    for i in input.windows(3) {
        let s: u32 = i.iter().sum();
        if s > last_value {
            increment_counter = increment_counter + 1;
        }
        last_value = s;
    }

    increment_counter
}
