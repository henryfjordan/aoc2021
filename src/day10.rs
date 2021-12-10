#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(
        |s| s.chars().collect()
    ).collect()
}

fn check_integrity(line: Vec<char>) -> (u32, Vec<char>) {
    let mut stack: Vec<char> = Vec::new();
    let opening_chars = vec!['{', '(', '[', '<'];

    for c in line {
        if opening_chars.contains(&c) {
            stack.push(c)
        } else {
            let last_bracket = stack.pop().unwrap();
            
            let is_illegal = match last_bracket {
                '{' => c != '}',
                '[' => c != ']',
                '<' => c != '>',
                '(' => c != ')',
                _ => false
            };

            if is_illegal {
                return (match c {
                    '}' => 1197,
                    ']' => 57,
                    '>' => 25137,
                    ')' => 3,
                    _ => 0
                }, stack)
            }
        }
    }

    (0, stack)
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;

    for line in input {
        score += check_integrity(line.to_vec()).0;
    }

    score
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<char>>) -> u32 {
    let mut scores = Vec::new();

    for line in input {
        let (score, stack) = check_integrity(line.to_vec());
        if score == 0 {
            let mut total_score = 0;
            let mut reverse_stack = stack.clone();
            reverse_stack.reverse();
            for opener in reverse_stack {
                let this_one_score = match opener {
                    '{' => 3,
                    '[' => 2,
                    '<' => 4,
                    '(' => 1,
                    _ => 0
                };

                total_score = (total_score * 5) + this_one_score;
            }
            scores.push(total_score);
        }
    }

    scores.sort();
    println!("{:?}", scores);
    scores[scores.len() / 2]
}