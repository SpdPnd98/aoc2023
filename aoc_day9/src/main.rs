use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    // let inputs = parse_input1(filename);

    // parse the input file
    let inputs = parse_input2(filename);

    // run the algorithm
    let result = solve(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input2(filename: &String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_own = line.expect("not a line");
        let mut v = line_own.split_whitespace().map(|x| {x.parse::<i32>().expect("cannot be parsed into number")}).collect::<Vec<i32>>();
        v.reverse();
        result.push(v);
    }
    result
}

fn parse_input1(filename: &String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_own = line.expect("not a line");
        result.push(line_own.split_whitespace().map(|x| {x.parse::<i32>().expect("cannot be parsed into number")}).collect::<Vec<i32>>());
    }
    result
}

fn solve(input: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    
    for history in input {
        println!("New history: {:?}", history);
        let mut pyramid: Vec<Vec<i32>> = Vec::new();
        let mut curr = history.clone();
        pyramid.push(curr.clone());
        while !curr.iter().fold(true, |prev, curr| {prev && curr == &0}) {
            curr = curr[0..curr.len()-1].iter().zip(&curr[1..curr.len()]).map(|x| {x.1 - x.0}).collect::<Vec<i32>>();
            pyramid.push(curr.clone());
        }

        // extrapolate
        let mut prev: Vec<i32>;
        let mut prev_val = 0;
        while pyramid.len() > 0 {
            prev = pyramid.pop().unwrap();
            println!("{:?}", prev);
            prev_val += prev[prev.len() - 1];
        }
        println!("Result here is {}", prev_val);
        result += prev_val;
    }
    result
}

