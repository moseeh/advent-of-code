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
    let mut total: i64 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let n = chars.len();
        let keep = 12; // Number of batteries to turn on
        let skip = n - keep; // Number of batteries to skip

        let mut result = String::new();
        let mut skipped = 0;
        let mut i = 0;

        // Greedy approach: at each position, pick the largest digit
        // that still allows us to pick enough digits afterwards
        while result.len() < keep {
            let max_skip_here = skip - skipped;

            // Find the largest digit in the window where we can still pick enough digits
            let mut max_char = '0';
            let mut max_idx = i;

            for j in i..=(i + max_skip_here) {
                if chars[j] > max_char {
                    max_char = chars[j];
                    max_idx = j;
                }
            }

            // Add the largest digit we found
            result.push(max_char);

            // Update how many we've skipped
            skipped += max_idx - i;
            i = max_idx + 1;
        }

        let value: i64 = result.parse().unwrap();
        total += value;
    }

    println!("{}", total);
}
