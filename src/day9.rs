use std::collections::{HashSet};

type Point = (usize, usize);

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(
        |s| s.chars().map(
            |c| c.to_digit(10).unwrap() as i32
        ).collect()
    ).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut score = 0;

    let max_height = input.len() - 1;
    for (i, row) in input.iter().enumerate() {
        let max_width = row.len() - 1;

        for (j, cell) in row.iter().enumerate() {
            if (
                (i == 0 || cell < &input[i-1][j]) &&
                (i == max_height || cell < &input[i+1][j]) &&
                (j == 0 || cell < &input[i][j-1]) &&
                (j == max_width || cell < &input[i][j+1])
            ) {
                score += 1 + cell;
            }
        }
    }

    score
}

fn calculate_basin_size(input: &Vec<Vec<i32>>, starting_point: Point, max_height: usize, max_width: usize) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = Vec::new();

    stack.push(starting_point);

    let mut basin_size = 0;

    while let Some(p) = stack.pop() {
        if !visited.contains(&p) {
            visited.insert(p);
            if input[p.0][p.1] < 9 {
                basin_size += 1;

                if p.0 > 0 {
                    stack.push((p.0 - 1, p.1));
                }

                if p.0 < max_height {
                    stack.push((p.0 + 1, p.1));
                }

                if p.1 > 0 {
                    stack.push((p.0, p.1 - 1));
                }                

                if p.1 < max_width {
                    stack.push((p.0, p.1 + 1));
                }
            }
        }
    }

    basin_size
}


#[aoc(day9, part2)]
fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut basin_sizes: Vec<i32> = Vec::new();

    let max_height = input.len() - 1;
    for (i, row) in input.iter().enumerate() {
        let max_width = row.len() - 1;

        for (j, cell) in row.iter().enumerate() {
            if (
                (i == 0 || cell < &input[i-1][j]) &&
                (i == max_height || cell < &input[i+1][j]) &&
                (j == 0 || cell < &input[i][j-1]) &&
                (j == max_width || cell < &input[i][j+1])
            ) {
                basin_sizes.push(calculate_basin_size(input, (i, j), max_height, max_width))
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}
