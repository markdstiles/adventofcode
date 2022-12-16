use std::fs;
use adventofcode::FindAll;

fn main() {
    let input = fs::read_to_string("day_5 input.txt").unwrap();

    let mut stacks = vec![
        vec![ 'H', 'T', 'Z', 'D' ],
        vec![ 'Q', 'R', 'W', 'T', 'G', 'C', 'S' ],
        vec![ 'P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H' ],
        vec![ 'L', 'C', 'N', 'F', 'H', 'Z' ],
        vec![ 'G', 'L', 'F', 'Q', 'S' ],
        vec![ 'V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S' ],
        vec![ 'Z', 'F', 'J' ],
        vec![ 'D', 'L', 'V', 'Z', 'R', 'H', 'Q' ],
        vec![ 'B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D' ],
    ];

    for line in input.lines() {
        if line.starts_with("move") {
            let m: Vec<String> = line.find_all("move {} from {} to {}").flatten().collect();
            
            let volume: u8 = m[0].parse().unwrap();
            let from: usize = m[1].parse().unwrap();
            let to: usize = m[2].parse().unwrap();

            println!("Moving {} from {} to {}", volume, from, to);

            //Part One
            /*for _ in 0..volume {
                let bx = stacks[from-1].pop().unwrap();
                stacks[to-1].push(bx);
            }*/

            //Part Two
            let mut temp_stack = vec!();
            for _ in 0..volume {    
                let bx = stacks[from-1].pop().unwrap();
                temp_stack.push(bx);
            }
            
            while let Some(bx) = temp_stack.pop() { stacks[to-1].push(bx); }
        }
    }

    for stack in stacks {
        println!("Top of stacks: [{}] ", stack.last().unwrap_or(&' '));
    }
}