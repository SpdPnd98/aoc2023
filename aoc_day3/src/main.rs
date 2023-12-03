use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    let inputs = parse_input(filename);

    // // run the algorithm
    // let result = solve1(inputs);

    // // print the result
    // println!("Result: {result}");

    // run the algorithm
    let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let r = BufReader::new(f);

    for line in r.lines() {
        result.push(line.unwrap().chars().collect::<Vec<char>>());
    }
    result
}

fn solve2(input: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for idy in 0..input.len() {
        for idx in 0..input[idy].len() {
            println!("Start index ({}, {})", idx, idy);
            if is_gear(input[idy][idx]) {
                let neighbours = generate_neighbour_coordinates_pt2(idx as i32, idy as i32, &input);
                let numeric_neighbours = neighbours.iter().filter(|pos| {is_numeric(input[pos.1 as usize][pos.0 as usize])}).collect::<Vec<&(i32, i32)>>();
                println!("Generated neighbours {:?}", neighbours);
                println!("Generated numeric neighbours {:?}", numeric_neighbours);

                let mut gear_ratio = 1;
                let mut num_items = 0;
                // expand each numeric neighbours so that a full digit is found
                let mut num = "".to_string();
                let mut to_explore = numeric_neighbours.clone();
                for n_pos in numeric_neighbours {
                    let x = n_pos.0 as usize;
                    let y = n_pos.1 as usize;

                    if to_explore.len() == 0 ||
                        to_explore.clone().into_iter().fold(true, |prev, curr| {prev && *curr != (x as i32, y as i32)}) {
                        continue;
                    }

                    // expand left
                    for idx_f in (0..x).rev() {
                        if is_numeric(input[y][idx_f]) {
                            num = input[y][idx_f].to_string() + &num;
                            to_explore.retain(|e| {**e != (idx_f as i32, y as i32)});
                        } else {
                            // it is no longer a valid number, stopping...
                            break;
                        }
                    }

                    //expand right
                    for idx_f in x..input[y].len() {
                        if is_numeric(input[y][idx_f]) {
                            num += &input[y][idx_f].to_string();
                            to_explore.retain(|e| {**e != (idx_f as i32, y as i32)});
                        } else {
                            // it is no longer a valid number, stopping...
                            break;
                        }
                    }

                    if num == "" {
                        println!("This number has been explored,")
                    } else {
                        println!("Item {} in gear to get ratio: {}", num_items + 1,num.parse::<i32>().unwrap());
                        gear_ratio *= num.parse::<i32>().unwrap();
                        num_items += 1;
                    }
                    num = "".to_string();
                }
                if num_items == 2 {
                    println!("Gear ratio is {}", gear_ratio);
                    sum += gear_ratio;
                } else {
                    println!("There are {} items beside the gear, skipping...", num_items);
                }
                num_items = 0;
            }
        }
    }
    sum
}

fn is_gear(e: char) -> bool {
    // the character is *
    e == '*'
}

fn generate_neighbour_coordinates_pt2(x:i32, y:i32, visited_map: &Vec<Vec<char>>) -> Vec<(i32, i32)>{
    vec![(x-1,y-1), (x,y-1), (x+1,y-1),
         (x-1,y),   (x,y),   (x+1,y),
         (x-1,y+1), (x,y+1), (x+1,y+1)].into_iter().filter(|pos| {
        if (pos.0 < 0 || pos.0 >= visited_map[0].len() as i32) && (pos.1 < 0 || pos.1 >= visited_map.len() as i32) {
            false
        } else {
            true
        }
     }).collect::<Vec<(i32,i32)>>()
}

fn solve1(input: Vec<Vec<char>>) -> i32 {
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for _ in 0..input.len() {
        let mut sub_arr: Vec<bool> = Vec::new();
        for _ in 0..input[0].len() {
            sub_arr.push(false);
        }
        visited.push(sub_arr);
    }

    let mut sum = 0;
    for idy in 0..input.len() {
        for idx in 0..input[idy].len() {
            println!("Start index ({}, {})", idx, idy);
            if !visited[idy][idx] && is_special_char(input[idy][idx]) {
                let neighbours = generate_neighbour_coordinates(idx as i32, idy as i32, &mut visited);
                let numeric_neighbours = neighbours.iter().filter(|pos| {is_numeric(input[pos.1 as usize][pos.0 as usize])}).collect::<Vec<&(i32, i32)>>();

                // expand each numeric neighbours so that a full digit is found
                let mut num = "".to_string();
                for n_pos in numeric_neighbours {
                    let x = n_pos.0 as usize;
                    let y = n_pos.1 as usize;
                    // expand left
                    for idx_f in (0..x).rev() {
                        if !visited[y][idx_f] && is_numeric(input[y][idx_f]) {
                            visited[y][idx_f] = true;
                            num = input[y][idx_f].to_string() + &num;
                        } else {
                            // it is no longer a valid number, stopping...
                            break;
                        }
                    }

                    //expand right
                    for idx_f in x..input[y].len() {
                        if !visited[y][idx_f] && is_numeric(input[y][idx_f]) {
                            visited[y][idx_f] = true;
                            num += &input[y][idx_f].to_string();
                        } else {
                            // it is no longer a valid number, stopping...
                            break;
                        }
                    }

                    if num == "" {
                        println!("This number has been explored,")
                    } else {
                        sum += num.parse::<i32>().unwrap();
                        println!("Found num at ({}, {}): {}\n", x, y, num);
                    }
                    num = "".to_string();
                }
            }
        }
    }
    sum
}

fn is_special_char(e: char) -> bool {
    // the character is not a dot and is not in the range 0-9
    e != '.' && !(e >= '0' && e <= '9')
}

fn is_numeric(e: char) -> bool {
    // the character is a number
    // println!("Is {} between 0 and 9? {}", e, e >= '0' && e <= '9');
    e >= '0' && e <= '9'
}

fn generate_neighbour_coordinates(x:i32, y:i32, visited_map: &mut Vec<Vec<bool>>) -> Vec<(i32, i32)>{
    vec![(x-1,y-1), (x,y-1), (x+1,y-1),
         (x-1,y),   (x,y),   (x+1,y),
         (x-1,y+1), (x,y+1), (x+1,y+1)].into_iter().filter(|pos| {
        if ((pos.0 < 0 || pos.0 >= visited_map[0].len() as i32) && (pos.1 < 0 || pos.1 >= visited_map.len() as i32)) || visited_map[pos.1 as usize][pos.0 as usize] {
            false
        } else {
            true
        }
     }).collect::<Vec<(i32,i32)>>()
}