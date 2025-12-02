use crate::utils::parse;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Invalid part"),
    }
}

fn part1() {
    let mut position: i32 = 50;
    let mut count = 0;
    let lines = parse::load_lines("src/inputs/day_01.txt");

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let direction = &line[0..1];
        let val: i32 = line[1..].parse().unwrap();
        let delta = match direction {
            "R" => val,
            "L" => -val,
            _ => panic!("Invalid direction"),
        };
        position = (position + delta).rem_euclid(100);

        if position == 0 {
            count += 1;
        }
    }
    println!("Solution to day 1 part 1 {}", count)
}

fn part2() {
    let mut position: i32 = 50;
    let mut count = 0;
    let lines = parse::load_lines("src/inputs/day_01.txt");

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let direction = &line[0..1];
        let val: i32 = line[1..].parse().unwrap();
        let delta = match direction {
            "R" => val,
            "L" => -val,
            _ => panic!("Invalid direction"),
        };
        let new_position = (position + delta).rem_euclid(100);

        // Count how many times we passed through 0
        let mut passes = (position + delta).div_euclid(100);
        // If the final position is exactly 0, it counts as a pass
        if position + delta == 0 {
            passes += 1;
        }
        if (position + delta) < 0 && position == 0 {
            passes += 1;
        }
        if (new_position == 0) && (position + delta) < 0 {
            passes -= 1;
        }

        count += passes.abs();

        position = new_position;
    }
    println!("Solution to day 1 part 2 {}", count)
}
