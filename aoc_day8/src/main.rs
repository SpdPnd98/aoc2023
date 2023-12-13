use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

static mut current_v_ind: i32 = -1;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // parse the input file
    let inputs = parse_input(filename);

    // run the algorithm
    // let result = solve(inputs);

    // run the algorithm
    let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut result: HashMap<String, (String, String)> = HashMap::new();
    let mut sequence: Vec<char> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut line_num = 1;

    for line in reader.lines() {
        if line_num == 1{
            sequence = line.expect("string").chars().collect::<Vec<char>>();
            println!("Seqeunce is {:?}, len {}", sequence, sequence.len());
        } else if line_num == 2 {
            // this should be an empty line
            println!("Empty line")
        } else {
            let line_own = line.expect("string");
            let line_split = line_own.split('=').collect::<Vec<&str>>();
            let res = line_split[1].trim()[1..9].split(',').map(|x|{x.trim()}).collect::<Vec<&str>>();
            let key: &str = line_split[0].trim();
            result.insert(key.to_string(), (res[0].to_string(), res[1].to_string()));
            // println!("{:?}", res);
        }
        line_num += 1;
    }
    (sequence, result)
}

fn solve2(input: (Vec<char>, HashMap<String, (String, String)>)) -> i128 {
    let seq: Vec<char> = input.0;
    let graph: HashMap<String, (String, String)> = input.1;

    let mut num_steps = 0;

    let mut current: Vec<String> = graph.keys().into_iter()
                                    .filter(|x: &&String| {x.chars().collect::<Vec<char>>()[2] == 'A'})
                                    .map(|x: &String| {x.clone()}).collect::<Vec<String>>();


    let mut path_costs: Vec<i128> = Vec::new();

    while current.len() > 0{
        // get current as a copy here first
        let mut current_vs = current.clone();
        let next_char: char = get_next_step(&seq);
        
        // do operation
        for (id, curr) in current.iter().enumerate() {
            let current_branch: (String, String) = graph.get(curr).unwrap().clone();
            if next_char == 'R' {
                current_vs[id] = current_branch.1;
            } else {
                current_vs[id] = current_branch.0;
            }
        }

        println!("{:?} at step {} becomes {:?}", current, num_steps, current_vs);
        num_steps += 1;
        
        let mut remaining_vs: Vec<String> = Vec::new();
        for c in current_vs {
            if c.chars().collect::<Vec<char>>()[2] == 'Z'{
                path_costs.push(num_steps);
            } else {
                remaining_vs.push(c);
            }
        }


        // set it back here
        current = remaining_vs;
    }
    println!("path costs: {:?}", path_costs);
    lcm(&path_costs)
}

fn lcm(nums: &[i128]) -> i128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd_of_two_numbers(b, a % b)
    }
}


fn solve(input: (Vec<char>, HashMap<String, (String, String)>)) -> i32 {
    let seq: Vec<char> = input.0;
    let graph: HashMap<String, (String, String)> = input.1;

    let mut num_steps = 0;

    let mut current: String = "AAA".to_string();
    while current != "ZZZ".to_string() {
        let next_char: char = get_next_step(&seq);
        let current_branch: &(String, String) = graph.get(&current).unwrap();
        if next_char == 'R' {
            current = current_branch.1.clone();
        } else {
            current = current_branch.0.clone();
        }
        num_steps += 1;
        // println!("{} at step {} with action {}", current, num_steps, next_char);
    }
    num_steps
}

fn get_next_step(seq: &Vec<char>) -> char {
    unsafe { current_v_ind = (current_v_ind + 1) % seq.len() as i32;
        if current_v_ind == 0 {
            // println!("Resetting current_v_ind...\n");
        } }
    seq[unsafe { current_v_ind as usize}]
}
