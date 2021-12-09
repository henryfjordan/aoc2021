fn binary_string_to_int(input: &str) -> u32 {
    u32::from_str_radix(input, 2).unwrap()
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<u32> {
    input.lines().map(binary_string_to_int).collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<u32>) -> u32 {
    let mut digit_counter = vec![0; 12];

    for j in 0..12 {
        for i in input {
            if u32::pow(2, j as u32) & i > 0 {
                digit_counter[j] += 1;
            }
        }
    }
    digit_counter.reverse();

    let mut gamma = 0;
    for &i in digit_counter.iter() {
        gamma += (i >= input.len() / 2) as u32;
        gamma = gamma << 1
    }
    gamma = gamma >> 1;

    let mut epsilon = 0;
    for &i in digit_counter.iter() {
        epsilon += (i <= input.len() / 2) as u32;
        epsilon = epsilon << 1
    }
    epsilon = epsilon >> 1;

    gamma * epsilon
}

// #[aoc(day3, part2)]
// fn part2(input: &Vec<u32>) -> u32 {

    
//     0
// }