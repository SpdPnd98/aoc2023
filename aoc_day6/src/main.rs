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

    // run the algorithm
    // let result = solve1(inputs);

     // parse the input file
     let inputs = parse_input2(filename);

     // run the algorithm
     let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input1(filename: &String) -> Vec<(i64, i64)> {
    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut times : Vec<i64> = Vec::new();
    let mut dist : Vec<i64> = Vec::new();

    let mut is_first = true;
    for line in reader.lines() {
        let line_own = line.expect("Is String");

        let mut num = "".to_string();
        for c in line_own.chars() {
            if is_first {
                if !is_numeric(c) && num != "".to_string() {
                    // a number exist
                    times.push(num.parse::<i64>().expect("num is not parsable"));
                    num = "".to_string();
                } else if !is_numeric(c) && num == "".to_string() {
                    continue;
                } else {
                    // can add to num
                    num += &c.to_string();
                }
            } else {
                if !is_numeric(c) && num != "".to_string() {
                    // a number exist
                    dist.push(num.parse::<i64>().expect("num is not parsable"));
                    num = "".to_string();
                } else if !is_numeric(c) && num == "".to_string() {
                    continue;
                } else {
                    // can add to num
                    num += &c.to_string();
                }
            }
        }
        is_first = false;
    }
    times.into_iter().zip(dist.into_iter()).collect::<Vec<(i64,i64)>>()
}

fn is_numeric(e: char) -> bool {
    e >= '0' && e <= '9'
}

fn solve2(input: (i64, i64)) -> i64 {
    let mut ways = 0;
    for t in 0..input.0 {
        if t * (input.0-t) > input.1 {
            ways += 1;
        }
    }
    ways
}

fn parse_input2(filename: &String) -> (i64, i64){
    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut times : i64 = -1;
    let mut dist : i64 = -1;

    for line in reader.lines() {
        let line_own = line.expect("Is String");

        let mut num = "".to_string();
        for c in line_own.chars() {
            if !is_numeric(c) {
                continue;
            } else {
                // can add to num
                num += &c.to_string();
            }
        }
        if times < 0 {
            times = num.parse::<i64>().expect("num is not parsable");
        } else {
            dist = num.parse::<i64>().expect("num is not parsable");
        }
    }
    (times, dist)
}

fn solve1(input: Vec<(i64, i64)>) -> i64 {
    let mut res = 1;
    for val in input {
        let mut ways = 0;
        for t in 0..val.0 {
            if t * (val.0-t) > val.1 {
                ways += 1;
            }
        }
        res *= ways;
    }
    res
}
