use std::cmp::max;
use std::collections::VecDeque;
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
    // let result = solve1(inputs);

    // run the algorithm
    let result = solve2(inputs);

    // print the result
    println!("Result: {result}");
}

fn parse_input(filename: &String) -> ((usize,usize), Vec<Vec<char>>) {
    let mut result: Vec<Vec<char>> = Vec::new();

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut col = 0;
    let mut row = 0;

    for (id, line) in reader.lines().enumerate() {
        let line_own = line.unwrap();
        let s_at = line_own.find('S');
        if s_at != None {
            col = s_at.unwrap();
            row = id;
        }
        result.push(line_own.chars().collect::<Vec<char>>());
    }
    println!("start at ({}, {})", row, col);
    ((row, col), result)
}

fn solve2(input: ((usize, usize), Vec<Vec<char>>)) -> i32 {
    // bfs
    let s_at = input.0;
    let mut inp_map = input.1;
    let mut costmap: Vec<Vec<i32>> = vec![vec![-1; inp_map[0].len()]; inp_map.len()];
    
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back(s_at);
    costmap[s_at.0][s_at.1] = 0;

    while frontier.len() > 0 {
        let next_coords = frontier.pop_front().expect("invalid coordinates");
        let curr_char = inp_map[next_coords.0][next_coords.1];
        let init: Vec<(i32, i32)> = vec![((next_coords.0) as i32 - 1 , (next_coords.1) as i32),
                                             ((next_coords.0) as i32 + 1, (next_coords.1) as i32),
                                             ((next_coords.0) as i32, (next_coords.1) as i32 - 1),
                                             ((next_coords.0) as i32, (next_coords.1) as i32 + 1)];
        let mut neighbours: Vec<(usize, usize)> = init.iter()
                        .filter(|(r, c)| {r >= &0 && c >= &0 && (*r as usize) < inp_map.len() && 
                            (*c as usize) < inp_map[0].len() && costmap[*r as usize][*c as usize] <= 0}).map(|x| { (x.0 as usize, x.1 as usize)}).collect::<Vec<(usize, usize)>>();
        
        // println!("\nneighbours at ({}, {}) are {:?}", next_coords.0, next_coords.1, neighbours);

        // set valid neighbours
        match curr_char {
            'S' => {
                neighbours = neighbours.iter().filter(|(r,c)| {
                    if c < &next_coords.1 {
                        // check left side
                        inp_map[*r][*c] != '|' &&
                        inp_map[*r][*c] != 'J' &&
                        inp_map[*r][*c] != '7' &&
                        inp_map[*r][*c] != '.'
                    } else if c > &next_coords.1 {
                        // check right side
                        inp_map[*r][*c] != '|' &&
                        inp_map[*r][*c] != 'L' &&
                        inp_map[*r][*c] != 'F' &&
                        inp_map[*r][*c] != '.'
                    } else if r > &next_coords.0 {
                        // check down side
                        inp_map[*r][*c] != '-' &&
                        inp_map[*r][*c] != 'F' &&
                        inp_map[*r][*c] != '7' &&
                        inp_map[*r][*c] != '.'
                    } else if r < &next_coords.0 {
                        // check up side
                        inp_map[*r][*c] != '-' &&
                        inp_map[*r][*c] != 'L' &&
                        inp_map[*r][*c] != 'J' &&
                        inp_map[*r][*c] != '.'
                    } else {
                        false
                    }

                }).map(|x| {*x}).collect::<Vec<(usize, usize)>>();

                // replace S

                let test_neighbours = neighbours.clone().iter().map(|x| {(x.0 as i32, x.1 as i32)}).collect::<Vec<(i32, i32)>>();

                if test_neighbours.contains(&((next_coords.0) as i32 - 1 , (next_coords.1) as i32)) && test_neighbours.contains(&((next_coords.0) as i32 + 1, (next_coords.1) as i32)) {
                    // contains down and up, replace with |
                    inp_map[next_coords.0][next_coords.1] = '|';
                } else if test_neighbours.contains(&((next_coords.0) as i32 - 1 , (next_coords.1) as i32)) && test_neighbours.contains(&((next_coords.0) as i32, (next_coords.1) as i32 - 1)) {
                    // contains down and left, replace with J
                    inp_map[next_coords.0][next_coords.1] = 'J';
                } else if test_neighbours.contains(&((next_coords.0) as i32 - 1 , (next_coords.1) as i32 )) && test_neighbours.contains(&((next_coords.0) as i32, (next_coords.1) as i32 + 1)) {
                    // contains down and right, replace with L
                    inp_map[next_coords.0][next_coords.1] = 'L';
                } else if test_neighbours.contains(&((next_coords.0) as i32 , (next_coords.1) as i32 - 1)) && test_neighbours.contains(&((next_coords.0) as i32, (next_coords.1) as i32 + 1)) {
                    // contains left and right, replace with -
                    inp_map[next_coords.0][next_coords.1] = '-';
                } else if test_neighbours.contains(&((next_coords.0) as i32 + 1 , (next_coords.1) as i32 )) && test_neighbours.contains(&((next_coords.0) as i32, (next_coords.1) as i32 - 1)) {
                    // contains up and left, replace with 7
                    inp_map[next_coords.0][next_coords.1] = '7';
                } else if test_neighbours.contains(&((next_coords.0) as i32 + 1 , (next_coords.1) as i32 )) && test_neighbours.contains(&((next_coords.0) as i32, (next_coords.1) as i32 + 1)) {
                    // contains up and right, replace with F
                    inp_map[next_coords.0][next_coords.1] = 'F';
                } else {
                    // neighbout does not contain enough information, throw error
                    println!("There seems to be an error! {:?} and {:?}", next_coords, neighbours);
                }
            },
            '|' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    next_coords.1 == *c &&
                    (next_coords.0 < *r && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.0 > *r && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'F' ))
                }).map(|x| {*x}).collect();
            },
            '-' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    next_coords.0 == *r &&
                    (next_coords.1 < *c && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.1 > *c && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'F' ))
                }).map(|x| {*x}).collect();
            },
            'L' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 < *c || next_coords.0 > *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'F' ))
                    
                }).map(|x| {*x}).collect();
            },
            'J' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 > *c || next_coords.0 > *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'F' || inp_map[*r][*c] == 'L' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'F' || inp_map[*r][*c] == '7' ))
                    
                }).map(|x| {*x}).collect();
            },
            '7' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 > *c || next_coords.0 < *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'F' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'J' ))
                    
                }).map(|x| {*x}).collect();
            },
            'F' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 < *c || next_coords.0 < *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'J' || inp_map[*r][*c] == '7' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'J' || inp_map[*r][*c] == 'L' ))
                    
                }).map(|x| {*x}).collect();
            },
            '.' => {
                // It should not be possible to be here
                neighbours = Vec::new();
            },
            _ => println!("Not valid")
        }

        println!("Neighbours at ({}, {}) {} are {:?}", next_coords.0, next_coords.1, inp_map[next_coords.0][next_coords.1], neighbours);
        for (r, c) in neighbours {
            costmap[r][c] = 11;
            frontier.push_back((r, c));
        }
    }

    let mut passed_through = false;
    let mut within_loop = 0;
    for i in 0..costmap.len() {
        passed_through = false;
        println!("Line: {}", i);
        let mut prev_char: char = '.';
        for j in 0..costmap[i].iter().rposition(|x| {*x != -1}).unwrap_or(0) {
            if costmap[i][j] == -1 {
                if passed_through {
                    within_loop += 1;
                    costmap[i][j] = -2;
                }
            } else {
                if is_connect_dont_maintain(inp_map[i][j], prev_char) {
                    println!("Direction changed, do not maintain {}, {}", inp_map[i][j], prev_char);
                    passed_through = !passed_through;
                } else {
                    println!("Does not change direction, maintain {}, {}", inp_map[i][j], prev_char);
                }
            }
            prev_char = if inp_map[i][j] == 'F' || inp_map[i][j] == 'L' || inp_map[i][j] == '7' || inp_map[i][j] == 'J' || inp_map[i][j] == '|' {inp_map[i][j]} else {prev_char};
        }
    }
    print_map(costmap);
    within_loop
}

fn is_connect_dont_maintain(origin:char, prev:char) -> bool {
    origin != '-' && (
    // these are straight lines, we flip them?
        (prev == '.' ) || (prev == '|' ) ||
        (origin == '|') ||
        // already flipped these combinations when at prev_char
        ! (
        (origin == '7' && prev == 'L') ||
        (origin == 'J' && prev == 'F')  )
    )

}

fn is_connect_maintain(origin:char, prev:char) -> bool {
    (origin == 'J' && prev == 'F') ||
    (origin == '7' && prev == 'L')
}


fn solve1(input: ((usize, usize), Vec<Vec<char>>)) -> i32 {
    // bfs
    let s_at = input.0;
    let inp_map = input.1;
    let mut costmap: Vec<Vec<i32>> = vec![vec![-1; inp_map[0].len()]; inp_map.len()];
    
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back(s_at);
    costmap[s_at.0][s_at.1] = 0;

    let mut max_curr = 0;

    while frontier.len() > 0 {
        let next_coords = frontier.pop_front().expect("invalid coordinates");
        let curr_char = inp_map[next_coords.0][next_coords.1];
        let init: Vec<(i32, i32)> = vec![((next_coords.0) as i32 - 1 , (next_coords.1) as i32),
                                             ((next_coords.0) as i32 + 1, (next_coords.1) as i32),
                                             ((next_coords.0) as i32, (next_coords.1) as i32 - 1),
                                             ((next_coords.0) as i32, (next_coords.1) as i32 + 1)];
        let mut neighbours: Vec<(usize, usize)> = init.iter()
                        .filter(|(r, c)| {r >= &0 && c >= &0 && (*r as usize) < inp_map.len() && 
                            (*c as usize) < inp_map[0].len() && costmap[*r as usize][*c as usize] <= 0}).map(|x| { (x.0 as usize, x.1 as usize)}).collect::<Vec<(usize, usize)>>();
        
        // println!("\nneighbours at ({}, {}) are {:?}", next_coords.0, next_coords.1, neighbours);

        // set valid neighbours
        match curr_char {
            'S' => {
                neighbours = neighbours.iter().filter(|(r,c)| {
                    if c < &next_coords.1 {
                        // check left side
                        inp_map[*r][*c] != '|' &&
                        inp_map[*r][*c] != 'J' &&
                        inp_map[*r][*c] != '7' &&
                        inp_map[*r][*c] != '.'
                    } else if c > &next_coords.1 {
                        // check right side
                        inp_map[*r][*c] != '|' &&
                        inp_map[*r][*c] != 'L' &&
                        inp_map[*r][*c] != 'F' &&
                        inp_map[*r][*c] != '.'
                    } else if r > &next_coords.0 {
                        // check down side
                        inp_map[*r][*c] != '-' &&
                        inp_map[*r][*c] != 'F' &&
                        inp_map[*r][*c] != '7' &&
                        inp_map[*r][*c] != '.'
                    } else if r < &next_coords.0 {
                        // check up side
                        inp_map[*r][*c] != '-' &&
                        inp_map[*r][*c] != 'L' &&
                        inp_map[*r][*c] != 'J' &&
                        inp_map[*r][*c] != '.'
                    } else {
                        false
                    }

                }).map(|x| {*x}).collect::<Vec<(usize, usize)>>();
            },
            '|' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    next_coords.1 == *c &&
                    (next_coords.0 < *r && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.0 > *r && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'F' ))
                }).map(|x| {*x}).collect();
            },
            '-' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    next_coords.0 == *r &&
                    (next_coords.1 < *c && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.1 > *c && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'F' ))
                }).map(|x| {*x}).collect();
            },
            'L' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 < *c || next_coords.0 > *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'J' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == '7' || inp_map[*r][*c] == 'F' ))
                    
                }).map(|x| {*x}).collect();
            },
            'J' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 > *c || next_coords.0 > *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'F' || inp_map[*r][*c] == 'L' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'F' || inp_map[*r][*c] == '7' ))
                    
                }).map(|x| {*x}).collect();
            },
            '7' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 > *c || next_coords.0 < *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'F' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'L' || inp_map[*r][*c] == 'J' ))
                    
                }).map(|x| {*x}).collect();
            },
            'F' => {
                neighbours = neighbours.iter().filter(|(r, c)| {
                    (next_coords.1 < *c || next_coords.0 < *r) &&
                    (next_coords.0 == *r && (inp_map[*r][*c] == '-' || inp_map[*r][*c] == 'J' || inp_map[*r][*c] == '7' ) ||
                     next_coords.1 == *c && (inp_map[*r][*c] == '|' || inp_map[*r][*c] == 'J' || inp_map[*r][*c] == 'L' ))
                    
                }).map(|x| {*x}).collect();
            },
            '.' => {
                // It should not be possible to be here
                neighbours = Vec::new();
            },
            _ => println!("Not valid")
        }

        println!("Neighbours at ({}, {}) {} are {:?}", next_coords.0, next_coords.1, inp_map[next_coords.0][next_coords.1], neighbours);
        for (r, c) in neighbours {
            // if (r, c) == s_at {
            //     // generated loop
            //     println!("Found loop!");
            //     return max_curr;
            // }
            costmap[r][c] = costmap[next_coords.0][next_coords.1] + 1;
            max_curr = max(max_curr, costmap[r][c]);
            frontier.push_back((r, c));
        }
    }
    print_map(costmap);
    max_curr
}

fn print_map(map: Vec<Vec<i32>>) {
    for m in map {
        println!("{:?}", m);
    }
}
