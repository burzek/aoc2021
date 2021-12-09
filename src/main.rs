use std::fs;
//use crate::day4::{day4_task1_task2};
//use crate::day5::{day5_task1, day5_task2};
use crate::day6::{day6_task1, day6_task2};
use crate::day7::{day7_task1, day7_task2};
use crate::day9::{day9_task1, day9_task2};


//mod day1;
//mod day2;
//mod day3;
//mod day5;
mod day6;
mod day7;
mod day9;

fn main() {
    // day1_task1(read_input("src/inputs/day1.input"));
    // day1_task2(read_input("src/inputs/day1.input"));
    // day2_task1(read_input("src/inputs/day2.input"));
    // day2_task2(read_input("src/inputs/day2.input"));
    //day3_task1(read_input("src/inputs/day3.input"));
    //day3_task2(read_input("src/inputs/day3.input"));
    // day6_task1(read_input("src/inputs/day6.input"));
    // day6_task2(read_input("src/inputs/day6.input"));
    day9_task1(read_input("src/inputs/day9.input"));
    day9_task2(read_input("src/inputs/day9.input"));
}

pub fn read_input(file_name : &str) -> String {
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}
