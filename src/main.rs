use std::fs;
use crate::day1::{day1_task1, day1_task2};

mod day1;

fn main() {
    day1_task1(read_input("src/inputs/day1.input"));
    day1_task2(read_input("src/inputs/day1.input"));
}

pub fn read_input(file_name : &str) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}
