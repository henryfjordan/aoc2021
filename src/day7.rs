
fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}


#[aoc_generator(day7)]
fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(str::parse).map(Result::unwrap).collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    let median = median(&mut input.clone());

    println!("{:?}", median);

    let mut fuel = 0;
    for i in input {
        fuel += (i - median).abs();
    }

    fuel
}

#[aoc(day7, part2)]
fn part2(input: &Vec<i32>) -> i32 {
    let mean = mean(&mut input.clone()) as i32;

    let mut fuel = 0;
    for i in input {
        let distance = (i - mean).abs();
        let fuel_needed: i32 = (1..(distance+1)).sum();
        fuel += fuel_needed;
    }

    fuel
}