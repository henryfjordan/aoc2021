#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<String>) -> u32 {
    let mut depth = 0;
    let mut horiz = 0;

    for i in input {
        let mut split = i.split(" ").take(2);
        let command: String = split.next().unwrap().to_string();
        let magnitude = split.next().unwrap().parse::<u32>().unwrap();
        match command.as_ref() {
            "forward" => {
                horiz = horiz + magnitude
            },
            "up" => {
                depth = depth - magnitude 
            },
            "down" => {
                depth = depth + magnitude 
            },
            _ => println!("oops")
        }
    }

    horiz * depth
}

#[aoc(day2, part2)]
fn part2(input: &Vec<String>) -> u32 {
    let mut depth = 0;
    let mut horiz = 0;
    let mut aim = 0;

    for i in input {
        let mut split = i.split(" ").take(2);
        let command: String = split.next().unwrap().to_string();
        let magnitude = split.next().unwrap().parse::<u32>().unwrap();
        match command.as_ref() {
            "forward" => {
                horiz = horiz + magnitude;
                depth = depth + aim * magnitude;
            },
            "up" => {
                // depth = depth - magnitude;
                aim = aim - magnitude;
            },
            "down" => {
                // depth = depth + magnitude;
                aim = aim + magnitude;
            },
            _ => println!("oops")
        }
    }

    horiz * depth
}