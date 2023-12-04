use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

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

fn parse_input(filename: &String) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut result = Vec::new();

    let f = fs::File::open(filename).expect("Unable to open file");
    let r = BufReader::new(f);

    for line in r.lines() {
        let lip = line.expect("Should be a string");
        let li = lip.split(':').collect::<Vec<&str>>()[1].split('|').collect::<Vec<&str>>();
        if li.len() == 2 {
            let r = li[0].trim().split(' ').filter(|e| {e != &""}).map(|e| {e.trim().parse::<i32>().expect("Could not parse results r")}).collect::<Vec<i32>>();
            let l = li[1].trim().split(' ').filter(|e| {e != &""}).map(|e| {e.trim().parse::<i32>().expect("Could not parse results l")}).collect::<Vec<i32>>();
            result.push((r, l));
        } else {
            println!("Parse error!")
        }

    }

    result
}

fn solve2(input: Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut num_lotto: Vec<i32> = Vec::new();
    for _i in 0..input.len(){
        num_lotto.push(1); // start with 1 lotto
    }
    for (i, lotto) in input.clone().into_iter().enumerate() {
        let mut win_set = HashSet::new();
        let mut wins = 0;
        for winnings in lotto.1 {
            win_set.insert(winnings);
        }

        for my_selection in lotto.0 {
            if win_set.contains(&my_selection) {
                wins += 1;
                println!("{} in {:?} -> adding wins to {}", my_selection, win_set, wins);
            }
        }

        if i == input.len() - 1 {
            println!("We are at the end, we don't need to care how many beneath we win!");
            break;
        }
        for id in i+1..usize::min(input.len(), i + wins+1) {
            num_lotto[id] = num_lotto[id] + num_lotto[i];
        }
    }
    println!("Lotto tickets: {:?}", num_lotto);
    num_lotto.iter().fold(0, |prev, current| {prev + current})
}

fn solve1(input: Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut sum = 0;
    for lotto in input {
        let mut win_set = HashSet::new();
        let mut wins = 0;
        for winnings in lotto.1 {
            win_set.insert(winnings);
        }

        for my_selection in lotto.0 {
            if win_set.contains(&my_selection) {
                wins += 1;
                println!("{} in {:?} -> adding wins to {}", my_selection, win_set, wins);
            }
        }
        if wins > 0 {
            sum += i32::pow(2, wins - 1);
            println!("")
        }
    }
    sum
}
