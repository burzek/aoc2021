use std::cmp::max;
use std::collections::HashMap;

fn parse_input(input: String) -> Vec<(i8)> {
    input.lines().next().unwrap().split(",").map(|val| { val.trim().parse::<i8>().unwrap() }).collect::<Vec<(i8)>>()
}


pub fn day6_task1(input: String) {
    let mut population = parse_input(input);

    for day in 1..80 + 1 {
        // println!("Population in day {}:{:?}", day, population);

        let mut new_pop = 0;
        population = population
            .iter()
            .map(|&l| {
                let ic = l - 1;
                if ic < 0 {new_pop = new_pop + 1};
                if ic < 0 {6} else {ic}
            })
            .collect();


        //add new
        for i in 0..new_pop {
            population.push(8);
        }
    }

    println!("day 6 task 1: {}", population.len());
}


pub fn day6_task2(input: String) {
    let mut population = parse_input(input);

    //not usable, took too much time

    for day in 1..(256 + 1) {
        // println!("Population in day {}:{:?}", day, population);

        let mut new_pop = 0;
        population = population
            .iter()
            .map(|&l| {
                let ic = l - 1;
                if ic < 0 { new_pop = new_pop + 1 };
                if ic < 0 { 6 } else { ic }
            })
            .collect();


        //add new
        for i in 0..new_pop {
            population.push(8);
        }
    }
    println!("day 6 task 2: {}", population.len());
}

