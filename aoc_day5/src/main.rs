use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::min;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    // let inputs = parse_input1(filename);

    // parse the input file
    let inputs = parse_input2(filename);

    // run the algorithm
    // let result = solve(inputs);

    // run the algorithm
    let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input1(filename: &String) -> (Vec<i64>, Vec<Vec<(i64,i64,i64)>>) {
    let mut result: Vec<Vec<(i64,i64,i64)>> = Vec::new();
    let mut intermediate: Vec<(i64, i64, i64)> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let r = BufReader::new(f);

    let mut seed_flag = false;
    let mut seeds: Vec<i64> = Vec::new();

    for line in r.lines() {
        let line_own = line.expect("line is a string");
        if !seed_flag {
            seed_flag = true;
            println!("Getting seeds");
            let mut skip_first = false;
            for num in line_own.split_whitespace().collect::<Vec<&str>>() {
                if !skip_first {
                    skip_first = true;
                    continue;
                } else {
                    seeds.push(num.parse::<i64>().unwrap());
                }
            } 
        }
        else if line_own == "".to_string() {
            println!("Proceeding to a new map");
            result.push(intermediate);
            intermediate = Vec::new();
            continue;
        } 
        else if !is_numeric(line_own.chars().collect::<Vec<char>>()[0]){
            println!("{}", line_own);
            continue;
        }
        else {
            // At this point it has to be just numbers
            let nums = line_own.split(' ').collect::<Vec<&str>>();
            intermediate.push((nums[0].parse::<i64>().unwrap(), nums[1].parse::<i64>().unwrap(), nums[2].parse::<i64>().unwrap()));
        }
    }

    (seeds, result)
}

fn parse_input2(filename: &String) -> (VecDeque<(i64, i64)>, Vec<Vec<(i64,i64,i64)>>) {
    let mut result: Vec<Vec<(i64,i64,i64)>> = Vec::new();
    let mut intermediate: Vec<(i64, i64, i64)> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let r = BufReader::new(f);

    let mut seed_flag = false;
    let mut seeds: VecDeque<(i64, i64)> = VecDeque::new();

    for line in r.lines() {
        let line_own = line.expect("line is a string");
        if !seed_flag {
            seed_flag = true;
            println!("Getting seeds");
            let mut skip_first = false;
            let mut starting_seed = -1;
            for num in line_own.split_whitespace().collect::<Vec<&str>>() {
                if !skip_first {
                    skip_first = true;
                    continue;
                } else if starting_seed < 0{
                    starting_seed = num.parse::<i64>().unwrap();
                } else {
                    let num_seeds = num.parse::<i64>().unwrap();
                    seeds.push_back((starting_seed, starting_seed + num_seeds));
                    starting_seed = -1;
                }
            } 
        }
        else if line_own == "".to_string() {
            println!("Proceeding to a new map");
            result.push(intermediate);
            intermediate = Vec::new();
            continue;
        } 
        else if !is_numeric(line_own.chars().collect::<Vec<char>>()[0]){
            println!("{}", line_own);
            continue;
        }
        else {
            // At this point it has to be just numbers
            let nums = line_own.split(' ').collect::<Vec<&str>>();
            intermediate.push((nums[0].parse::<i64>().unwrap(), nums[1].parse::<i64>().unwrap(), nums[2].parse::<i64>().unwrap()));
        }
    }

    (seeds, result)
}

fn solve2(input: (VecDeque<(i64,i64)>, Vec<Vec<(i64,i64,i64)>>)) -> i64 {
    let mut seeds = input.0;
    let maps = input.1;

    println!("The size of the maps is {}", maps.len());
    println!("The number of seeds is {}", seeds.len());

    println!("The seeds start as {:?}", seeds);

    for map in &maps {
        let mut seeds_result = VecDeque::new();
        while seeds.len() > 0 {
            // split is 4 cases:
            // 1. map contains seeds completely -> remove from seed, put in current seed
            // 2. seed contains map completely -> remove from seed, put in current seed, left with 1-2 other ranges and put back into seed
            // 3. map & seed have some arbitrary intersections -> remove from seed, put in current seed, left with 1 other range
            // 4. map & seed have no intersections at all -> remove from seed, put in current seed directly
            let current_seed_range = seeds.pop_front().unwrap();
            let result = split(current_seed_range, map.clone());

            // store the result temporarily
            for r in result.iter() {
                seeds_result.push_back(r.clone());
            }
        }
        println!("Doing next map..., seeds range: {:?}", seeds_result);
        seeds = seeds_result;
    }

    // At this point, seeds have been transfromed into a range of locations, find the smallest LHS of location ranges
    let mut current_smallest = i64::MAX;
    for seed in seeds {
        if current_smallest > seed.0 {
            current_smallest = seed.0;
        }
    }
    current_smallest
}

fn split(current_seed_range: (i64, i64), map: Vec<(i64, i64, i64)>) -> Vec<(i64, i64)> {
    // returns (has split, split range map result)

    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut seeds_split: Vec<(i64, i64)> = Vec::new();

    seeds.push(current_seed_range);

    while seeds.len() > 0 {
        let seed = seeds.pop().unwrap();
        let mut no_intersection = 0;
        for r in &map {
            let start = r.1;
            let end = r.1 + r.2;
            
            // case 1: map contains seed range completely
            if start <= seed.0 && end > seed.1 {
                seeds_split.push((seed.0 - start + r.0, seed.1 - start + r.0));
                break;
            } 
            
            // case 2: seed range contain map range completely
            else if start > seed.0 && end <= seed.1 {
                println!("Contained in range ({}, {}),  ({}, {})", seed.0, seed.1, start, end);
                seeds.push((seed.0, start));
                seeds.push((end, seed.1));
                seeds_split.push((r.0, r.0 + r.2));
                break;
            }

            // case 3: no intersection between map and seed range, check next
            else if start >= seed.1 || end <= seed.0 {
                println!("Outside of range  ({}, {}),  ({}, {})", seed.0, seed.1, start, end);
                no_intersection += 1;
            }

            // case 4: some arrbitrary intersection between map range and seed range
            else {
                println!("Arbitrary range ({}, {}),  ({}, {})", seed.0, seed.1, start, end);
                // case 4a; map left, seed range right
                if start < seed.0 && end <= seed.1 {
                    seeds_split.push((seed.0 - start + r.0, r.0 + r.2));
                    seeds.push((end, seed.1));
                    break;
                
                // case 4b: map right, seed range left
                } else if start >= seed.0 && end > seed.1 {
                    seeds_split.push((r.0, seed.1 - start + r.0));
                    seeds.push((seed.0, start));
                    break;

                } else {
                    println!("Should not reach this line! ({}, {}),  ({}, {})", seed.0, seed.1, start, end);
                }
            }
        }
        if no_intersection == map.len() {
            // case 5: no intersection for any map range, adding directly to mapping
            seeds_split.push((seed.0, seed.1));
        }
        
    }
    seeds_split
}

fn is_numeric(e: char) -> bool{
    '0' <= e && e <= '9' 
}

fn solve(input: (Vec<i64>, Vec<Vec<(i64,i64,i64)>>)) -> i64 {
    let seeds = input.0;
    let maps = input.1;

    let mut current_seed : i64;
    let mut current_lowest_seed = i64::MAX;

    println!("The size of the maps is {}", maps.len());
    println!("The number of seeds is {}", seeds.len());

    let mut map_counter = 1;
    for seed in seeds {
        current_seed = seed;
        // map_counter = 1;
        for map in &maps {
            for range in map {
                let start = range.1;
                let end = range.1 + range.2;
                // println!("Start:{}  End:{}", start, end);
                if current_seed >= start && current_seed < end {
                    current_seed = range.0 + (current_seed - range.1); // assume no overlaps
                    // println!("found range {} ", current_seed);
                    break; // found a range, proceed to next range
                }
            }
            // println!("{}", map_counter);
            // map_counter += 1;
        }
        // println!("{} seed maps to {} location", seed, current_seed);
        current_lowest_seed = min(current_lowest_seed, current_seed);
    }
    current_lowest_seed
}
