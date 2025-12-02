mod day_01;
mod day_02;

/// run the requested day and part
pub fn run(day: u8, part: u8) {
    match day {
        1 => day_01::run(part),
        2 => day_02::run(part),
        3..=12 => println!("Day {} is not implemented yet", day),
        _ => unreachable!(),
    }
}
