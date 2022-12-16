use std::collections::VecDeque;

fn unique(v: &VecDeque<char>) -> bool {
    for c in v {
        if v.iter().filter(|x| x == &c).take(2).count() > 1 {
            return false;
        }
    }

    true
}

fn main() {
    
    let input = std::fs::read_to_string("day_6 input.txt").unwrap();

    let buffer_size = 14;
    
    let mut buffer: VecDeque<char> = input[0..buffer_size].chars().collect();
    let mut is_unique = unique(&buffer);
    let mut index = buffer_size;

    while !is_unique && index < input.len() {
        buffer.pop_front();
        buffer.push_back(input.chars().nth(index).unwrap());
        is_unique = unique(&buffer);
        index += 1;
    }

    println!("Marker character index: {}", index);
}