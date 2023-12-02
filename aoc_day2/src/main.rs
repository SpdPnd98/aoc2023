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

    // run the algorithm
    // let result = solve1(inputs);

    // print the result
    // println!("Result: {result}");
    
    // run the algorithm
    let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) -> Vec<String> {
    let mut result = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let r = BufReader::new(f);

    for line in r.lines() {
        result.push(line.unwrap())
    }

    result
}

// Pt2
fn solve2(input: Vec<String>) -> i32 {
    let mut game_vec: Vec<HashMap<String, i32>> = Vec::new();
    for l in input {
        //Debug
        //println!("{}", l);
        
        let mut game_hashmap: HashMap<String, i32> = HashMap::new();
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let _ = l.split(": ").collect::<Vec<&str>>()[1].split("; ").map(|handful| {
            // Debug
            // println!("Found string: {}", handful);
            
            let _ = handful.split(", ").map(|grabbed| {
                let balls_taken = grabbed.split(" ").collect::<Vec<&str>>();
                let last_color_max = game_hashmap.get(balls_taken[1]);
                match last_color_max {
                    None => game_hashmap.insert(balls_taken[1].to_string(), balls_taken[0].to_string().parse::<i32>().unwrap()),
                    _ => game_hashmap.insert(balls_taken[1].to_string(), *[balls_taken[0].to_string().parse::<i32>().unwrap(), *last_color_max.expect("Last color isnt a number")].iter().max().unwrap()),
                };

                // Debug
                // println!("Checking for results {} : {}", balls_taken[1].to_string(), balls_taken[0].to_string());
            }).collect::<Vec<_>>();
            // println!("")
        }).collect::<Vec<_>>();
        game_vec.push(game_hashmap.clone());
    }

    let mut sum = 0;
    for game in game_vec.iter() {
        let red_cubes = game.get("red");
        let mut current_powerset = match red_cubes {
            None => 0, 
            _ => *red_cubes.unwrap(),
        };

        let green_cubes = game.get("green");
        current_powerset *= match green_cubes {
            None => 0, 
            _ => *green_cubes.unwrap(),
        };

        let blue_cubes = game.get("blue");
        current_powerset *= match blue_cubes {
            None => 0,
            _ => *blue_cubes.unwrap(),
        };

        sum += current_powerset;
    }
    println!("");
    sum as i32
}

// Pt 1
fn solve1(input: Vec<String>) -> i32 {
    let mut game_vec: Vec<HashMap<String, i32>> = Vec::new();
    for l in input {
        //Debug
        //println!("{}", l);
        
        let mut game_hashmap: HashMap<String, i32> = HashMap::new();
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let _ = l.split(": ").collect::<Vec<&str>>()[1].split("; ").map(|handful| {
            // Debug
            // println!("Found string: {}", handful);
            
            let _ = handful.split(", ").map(|grabbed| {
                let balls_taken = grabbed.split(" ").collect::<Vec<&str>>();
                let last_color_max = game_hashmap.get(balls_taken[1]);
                match last_color_max {
                    None => game_hashmap.insert(balls_taken[1].to_string(), balls_taken[0].to_string().parse::<i32>().unwrap()),
                    _ => game_hashmap.insert(balls_taken[1].to_string(), *[balls_taken[0].to_string().parse::<i32>().unwrap(), *last_color_max.expect("Last color isnt a number")].iter().max().unwrap()),
                };

                // Debug
                // println!("Checking for results {} : {}", balls_taken[1].to_string(), balls_taken[0].to_string());
            }).collect::<Vec<_>>();
            // println!("")
        }).collect::<Vec<_>>();
        game_vec.push(game_hashmap.clone());
    }

    let mut sum = 0;
    for (id, game) in game_vec.iter().enumerate() {
        let red_cubes = game.get("red");
        match red_cubes {
            _val if red_cubes != None && *red_cubes.unwrap() > 12 => continue, // bag must contain more than 12 red balls, this game is not valid
            _ => println!("Valid Red"),
        };

        let green_cubes = game.get("green");
        match green_cubes {
            _val if green_cubes != None && *green_cubes.unwrap() > 13 => continue, // bag must contain more than 12 red balls, this game is not valid
            _ => println!("Valid Green"),
        };

        let blue_cubes = game.get("blue");
        match blue_cubes {
            _val if blue_cubes != None && *blue_cubes.unwrap() > 14 => continue, // bag must contain more than 12 red balls, this game is not valid
            _ => println!("Valid Blue"),
        };

        sum += id + 1;
    }
    println!("");
    sum as i32
}
