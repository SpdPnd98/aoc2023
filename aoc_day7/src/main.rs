use std::env;
use std::fs;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    let inputs = parse_input(filename);

    // run the algorithm
    let result = solve(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) 
    -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let cards = HashMap::from(
        [('2', 1), ('3', 2), ('4', 3), ('5', 4), ('6', 5), ('7', 6), ('8', 7), ('9', 8),
         ('T', 9), ('J', 10), ('Q', 11), ('K', 12), ('A', 13)]);

    for line in reader.lines() {
        let line_own = line.unwrap();
        let vals = line_own.split(' ').collect::<Vec<&str>>();
        
        result.push(convert_to_comparable(vals[0].to_string(), vals[1].to_string(), &cards));
    }
    result
}

fn convert_to_comparable(cards: String, value: String, card_mapping: &HashMap<char, i32>) 
                -> (i32,i32,i32,i32,i32,i32,i32,i32,i32,i32,i32) {
    assert_eq!(cards.len(), 5);
    
    let p_val = value.parse::<i32>().expect("not a number");
    let mut map: HashMap<char, i32> = HashMap::new();

    for c in cards.chars() {
        if map.contains_key(&c) {
            let curr_c_occ = map.get(&c).unwrap();
            map.insert(c, curr_c_occ + 1);
        } else {
            map.insert(c, 1);
        }
    }

    let mut nums_stuff: Vec<i32> = Vec::from([0,0,0,0,0,0]);
    for k in map.keys() {
        let new_num = *map.get(k).expect("not a number");
        nums_stuff[new_num as usize] += 1;
    }

    let card_s = cards.chars().collect::<Vec<char>>();

    (nums_stuff[5],nums_stuff[4],nums_stuff[3],nums_stuff[2],nums_stuff[1],
    *card_mapping.get(&card_s[0]).expect("not a number"), 
    *card_mapping.get(&card_s[1]).expect("not a number"), 
    *card_mapping.get(&card_s[2]).expect("not a number"), 
    *card_mapping.get(&card_s[3]).expect("not a number"), 
    *card_mapping.get(&card_s[4]).expect("not a number"), 
    p_val)
}

fn solve(input: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)>) -> i32 {
    let mut inp: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32)> = Vec::new();
    input.clone_into(&mut inp);
    inp.sort();
    let mut result = 0;
    for (idx, i) in inp.iter().enumerate() {
        result += (idx as i32 + 1) * i.10;
        println!("Values are ({}, {:?}), result is: {}", idx as i32 + 1, i, result);
    }
    result
}

// 248154088 too low
// 249737949 too low