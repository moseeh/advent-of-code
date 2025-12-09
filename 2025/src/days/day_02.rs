use crate::utils::parse;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Invalid part"),
    }
}

fn part1() {
    let input = parse::load_file("src/inputs/day_02.txt");
    
    let mut sum: u64 = 0;
    for range in input.split(",") {
        let mut it = range.split("-");
        let start: u64 = it.next().unwrap().parse().unwrap();
        let end: u64 = it.next().unwrap().parse().unwrap();
        for id in start..=end {
            if is_invalid_id(id) {
                sum += id;
            }
        }
    }
    println!("{}", sum)

}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len %2 != 0 {
        return false;
    }
    let half = len / 2;
    &s[..half] == &s[half..]

}

fn part2() {
    println!("Running Day 2 Part 2");
}
