use crate::utils::parse;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Invalid part"),
    }
}

pub fn part1() {
    let lines = parse::load_lines("src/inputs/day_03.txt");
    let mut total = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut index_largest = 0;

        let mut largest = '0';
        let mut second_largest = '0';

        for (i, c) in chars.iter().enumerate() {
            if *c > largest {
                largest = *c;
                index_largest = i;
            }
        }

        if index_largest == chars.len() - 1 {
            second_largest = largest;
            largest = '0';
            for (i, c) in chars.iter().enumerate() {
                if *c > largest && i != chars.len() - 1 {
                    largest = *c;
                }
            }
        } else {
            for c in chars.iter().skip(index_largest + 1) {
                if *c > second_largest {
                    second_largest = *c;
                }
            }
        }

        let first = largest.to_digit(10).unwrap();
        let second = second_largest.to_digit(10).unwrap();
        total += first * 10 + second
    }

    println!("{}", total)
}
pub fn part2() {
    let lines = parse::load_lines("src/inputs/day_03.txt");
}
