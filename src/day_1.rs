use std::fs;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    id: i32,
    calorie_count: i64
}

fn day_1() {
    let contents = fs::read_to_string("day_1 input.txt").unwrap();

    let mut elves = vec!();
    let mut curr_elf = 1;
    let mut curr_total_cal = 0;
    
    for line in contents.lines() {
        if line.trim().is_empty() {
            elves.push(Elf { id: curr_elf, calorie_count: curr_total_cal });
            
            curr_elf += 1;
            curr_total_cal = 0;
        } else {
            let n: i64 = line.parse().unwrap();
            curr_total_cal += n;
        }
    }

    elves.sort_by(|a, b| b.calorie_count.cmp(&a.calorie_count));
    let max_elf = elves.first().unwrap();

    println!("Elf with highest calories is {} with calorie count of {}.", max_elf.id, max_elf.calorie_count);
    println!("Top three elves are...");
    let mut total = 0;
    for elf in elves.iter().take(3) {
        total += elf.calorie_count;
        println!("Elf number {} with calorie count {}", elf.id, elf.calorie_count);
    }
    println!("Total calorie count of {}", total);
}
