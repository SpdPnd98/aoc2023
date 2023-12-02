use std::env;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    let inputs = parse_input(filename);

    // run pt1
    // let result = solve1(inputs);

    // print the result for pt1
    // println!("Result for pt1: {result}");

    // run pt2
    let result = solve2(inputs);

    // print the result for pt2
    println!("Result for pt2: {result}");
}

fn parse_input(filename: &String) -> Vec<String> {
    let mut result = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}

fn solve2(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut linenum = 0;
    for l in input {
        linenum += 1;
        let v: Vec<char> = l.chars().collect();
        let mut num = "".to_string();

        // find leftmost and rightmost digits first
        let mut min_left_ind = l.len() as i32;
        let mut min_right_ind = 0;
        let mut left_num = "".to_string();
        let mut right_num = "".to_string();     
        for (id, c) in l.chars().collect::<Vec<char>>().into_iter().enumerate() {
            if is_ascii_num_checks(c) {
                min_left_ind = id as i32;
                left_num = c.to_string();
                break;
            }
        }
        for (id, c) in l.chars().rev().collect::<Vec<char>>().into_iter().enumerate() {
            if is_ascii_num_checks(c) {
                min_right_ind = (l.len() - 1 - id) as i32;
                right_num = c.to_string();
                break;
            }
        }

        // find leftmost and rightmost numbers
        let numbers = HashMap::from([("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")]);
        let left_most_num_p = numbers.keys().into_iter().map(|e| {
            let ind = l.find(e);
            match ind {
                None => (l.len() as i32, ""),
                _ => (ind.unwrap() as i32, *(numbers.get(e).unwrap()))
            }
        }).min().unwrap();

        let right_most_num_p = numbers.keys().into_iter().map(|e| {
            let ind = l.rfind(e);
            match ind {
                None => (-1 as i32, ""),
                _ => (ind.unwrap() as i32, *(numbers.get(e).unwrap()))
            }
        }).max().unwrap();

        if min_left_ind < left_most_num_p.0 {
            num += &left_num;
        } else {
            num += left_most_num_p.1;
        }
        if min_right_ind > right_most_num_p.0 {
            num += &right_num;
        } else {
            num += right_most_num_p.1;
        }
        println!("{} produced {}", l, num);
        sum += num.parse::<i32>().unwrap();
    }
    sum
}

fn is_ascii_num_checks(e: char) -> bool {
    e >= '1' && e <= '9'
}

// PT1
fn solve1(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut linenum = 0;
    for l in input {
        linenum += 1;
        let v: Vec<char> = l.chars().collect();
        let numbers: Vec<&char> = v.iter().filter(is_ascii_num).collect();
        let my_num: String = (**(numbers.first().unwrap())).to_string() + &(**(numbers.last().unwrap())).to_string();
        sum += my_num.parse::<i32>().unwrap();
    }
    sum
}

fn is_ascii_num(e: &&char) -> bool {
    **e >= '1' && **e <= '9'
}
