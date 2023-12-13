use priority_queue::PriorityQueue;
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    // get the arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // parse the input file
    let inputs = parse_input(filename);

    // run the algorithm
    let result = solve1(inputs.clone());

    // print the result
    println!("Result (wrong): {result}");

    // run the algorithm
    let result = solve(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        result.push(line.unwrap().chars().collect::<Vec<char>>());
    }
    result
}

fn solve1(input: Vec<Vec<char>>) -> i64 {
    let mut rows_empty: Vec<usize>  = vec![];
    let mut cols_empty: Vec<usize>  = vec![];
    let mut stars: Vec<(usize, usize)> = vec![];
    // let mut costmap: Vec<Vec<i64>> = vec![vec![1; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != '.' {
                stars.push((i, j));
            }
        }
    }

    for i in 0..input.len() {
        let mut is_empty = true;
        for j in 0..input[i].len() {
            if !is_empty {
                break;
            }
            is_empty = is_empty && input[i][j] == '.';
        }
        if is_empty {
            rows_empty.push(i);
        }
    }
    println!("Rows that are empty are {:?}", rows_empty);

    for i in 0..input[0].len() {
        let mut is_empty = true;
        for j in 0..input.len() {
            if !is_empty {
                break;
            }
            is_empty = is_empty && input[j][i] == '.';
        }
        if is_empty {
            cols_empty.push(i);
        }
    }
    println!("Cols that are empty are {:?}", cols_empty);

    let mut sum: i64 = 0;
    for (start_x, start_y) in stars.clone() {
        stars.remove(0);
        let mut frontier: PriorityQueue<(usize,usize), i64> = PriorityQueue::new();
        frontier.push( (start_x, start_y), 0);
        let mut costmap: Vec<Vec<i64>> = vec![vec![0; input[0].len()]; input.len()];
        costmap[start_x][start_y] = 0;
        while frontier.len() > 0 {
            let ((next_x, next_y), cost) = frontier.pop().unwrap(); 
            if costmap[next_x][next_y] > 0 {
                // already seen this node
                // println!("Already seen this node! {:?}", (next_x, next_y));
                continue;
            } else {
                costmap[next_x][next_y] = - cost;
                // println!("Popped ({}, {}): cost -> {}", next_x, next_y, -cost);
                if stars.contains(&(next_x, next_y)) {
                    sum = sum - cost;
                    println!("Adding cost from {:?} to {:?}: {}", (start_x, start_y),(next_x, next_y), -cost)
                }
            }
            let init: Vec<(i64, i64)> = vec![((next_x) as i64 - 1, (next_y) as i64),
                                             ((next_x) as i64 + 1, (next_y) as i64),
                                             ((next_x) as i64,     (next_y) as i64 - 1),
                                             ((next_x) as i64,     (next_y) as i64 + 1)];
            let neighbours: Vec<(usize, usize)> = init.iter()
                            .filter(|(r, c)| {r >= &0 && c >= &0 && (*r as usize) < costmap.len() && 
                                (*c as usize) < costmap[0].len() && costmap[*r as usize][*c as usize] == 0})
                            .map(|x| { (x.0 as usize, x.1 as usize)}).collect::<Vec<(usize, usize)>>();
            for n in neighbours {
                let mut n_cost = cost - 1;
                if n.0 != next_x && rows_empty.contains(&n.0) {
                    // move x dir
                    n_cost -= 9;
                }
                if n.1 != next_y && cols_empty.contains(&n.1) {
                    // move y dir
                    n_cost -= 9;
                }
                frontier.push(n, n_cost);
                // println!("Adding {:?} -> {:?}: {}", (start_x, start_y), n, -n_cost);
            }
        }
    }
    sum
}

fn solve(input: Vec<Vec<char>>) -> i64 {
    let mut rows_empty: Vec<usize>  = vec![];
    let mut cols_empty: Vec<usize>  = vec![];
    let mut stars: Vec<(usize, usize)> = vec![];
    // let mut costmap: Vec<Vec<i64>> = vec![vec![1; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != '.' {
                stars.push((i, j));
            }
        }
    }

    for i in 0..input.len() {
        let mut is_empty = true;
        for j in 0..input[i].len() {
            if !is_empty {
                break;
            }
            is_empty = is_empty && input[i][j] == '.';
        }
        if is_empty {
            rows_empty.push(i);
        }
    }
    println!("Rows that are empty are {:?}", rows_empty);

    for i in 0..input[0].len() {
        let mut is_empty = true;
        for j in 0..input.len() {
            if !is_empty {
                break;
            }
            is_empty = is_empty && input[j][i] == '.';
        }
        if is_empty {
            cols_empty.push(i);
        }
    }
    println!("Cols that are empty are {:?}", cols_empty);

    let mut sum: i64 = 0;
    for (start_x, start_y) in stars.clone() {
        stars.remove(0);
        let mut frontier: PriorityQueue<(usize,usize, Vec<(usize, usize)>), i64> = PriorityQueue::new();
        frontier.push( (start_x, start_y, vec![]) , 0);
        let mut costmap: Vec<Vec<i64>> = vec![vec![0; input[0].len()]; input.len()];
        costmap[start_x][start_y] = 0;
        let mut visited: Vec<(usize, usize)> = Vec::new();
        while frontier.len() > 0 && stars.len() > visited.len(){
            let ((next_x, next_y, path), cost) = frontier.pop().unwrap(); 
            if costmap[next_x][next_y] > 0 {
                // already seen this node
                // println!("Already seen this node! {:?}", (next_x, next_y));
                continue;
            } else {
                costmap[next_x][next_y] = - cost;
                // println!("Popped ({}, {}): cost -> {}", next_x, next_y, -cost);
                if stars.contains(&(next_x, next_y)) {
                    sum = sum - cost;
                    // println!("Adding cost from {:?} to {:?}: {} -> path {:?}", (start_x, start_y), (next_x, next_y), -cost, path);
                    println!("Adding cost from {:?} to {:?}: {}", (start_x, start_y), (next_x, next_y), -cost);
                    visited.push((next_x, next_y));
                }
            }

            let mut new_path = path.clone();
            new_path.push((next_x, next_y));
            let init: Vec<(i64, i64, Vec<(usize, usize)>)> = vec![((next_x) as i64 - 1, (next_y) as i64,     new_path.clone()),
                                                                  ((next_x) as i64 + 1, (next_y) as i64,     new_path.clone()),
                                                                  ((next_x) as i64,     (next_y) as i64 - 1, new_path.clone()),
                                                                  ((next_x) as i64,     (next_y) as i64 + 1, new_path.clone())];
            let neighbours: Vec<(usize, usize, Vec<(usize, usize)>)> = init.iter()
                            .filter(|(r, c, _)| {r >= &0 && c >= &0 && (*r as usize) < costmap.len() && 
                                (*c as usize) < costmap[0].len() && costmap[*r as usize][*c as usize] == 0})
                            .map(|x| { (x.0 as usize, x.1 as usize, x.2.clone())}).collect::<Vec<(usize, usize, Vec<(usize, usize)>)>>();
            for n in neighbours {
                let mut n_cost = cost - 1;
                if n.0 != next_x && rows_empty.contains(&n.0) {
                    // move x dir
                    n_cost -= 9;
                }
                if n.1 != next_y && cols_empty.contains(&n.1) {
                    // move y dir
                    n_cost -= 9;
                }


                frontier.push(n, n_cost);
                // println!("Adding {:?} -> {:?}: {}", (start_x, start_y), n, -n_cost);
            }
        }
    }
    sum
}
