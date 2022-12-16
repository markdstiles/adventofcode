use std::fs;

fn get_range(pair: &str) -> (u32, u32) {
    let range: Vec<&str> = pair.split('-').collect();

    (range[0].parse().unwrap(), range[1].parse().unwrap())
}

fn surrounds(range1: (u32, u32), range2: (u32, u32)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1  // range1 surrounds range2
}

fn overlaps(range1: (u32, u32), range2: (u32, u32)) -> bool {
    range1.0 <= range2.1 && range1.1 >= range2.0 // range1 overlaps with range2
}

fn main() {
    let contents = fs::read_to_string("day_4 input.txt").unwrap();

    let mut surround_count = 0;
    let mut overlap_count = 0;
    for line in contents.lines() {
        
        let pairs: Vec<&str> = line.split(',').collect();
        let range1 = get_range(pairs[0]);
        let range2 = get_range(pairs[1]);

        //Part One
        if surrounds(range1, range2) || surrounds(range2, range1) {
            surround_count += 1;
        }

        //Part Two
        if overlaps(range1, range2) {
            overlap_count += 1;
        }
    }

    println!("Number of surrounds: {}", surround_count);
    println!("Number of overlaps: {}", overlap_count);
}