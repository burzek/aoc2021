use std::fs;
use crate::day3::{day3_task1, day3_task2};

// mod day1;
// mod day2;
mod day3;

fn main() {
    // day1_task1(read_input("src/inputs/day1.input"));
    // day1_task2(read_input("src/inputs/day1.input"));
    // day2_task1(read_input("src/inputs/day2.input"));
    // day2_task2(read_input("src/inputs/day2.input"));
    //day3_task1(read_input("src/inputs/day3.input"));
    day3_task2(read_input("src/inputs/day3.input"));
}

pub fn read_input(file_name : &str) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}
