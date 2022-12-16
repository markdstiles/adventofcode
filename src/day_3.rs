fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn main() {
    //Part One
    let contents = std::fs::read_to_string("day_3 input.txt").unwrap();
    let mut common_items = vec!();

    for line in contents.lines() {
        let len = line.len();
        
        assert!(len % 2 == 0);

        let c1 = &line[0..len / 2];
        let c2 = &line[len / 2..len];

        for c in c1.chars() {
            if c2.contains(c) {
                common_items.push(c);
                break;
            }
        }
    }

    let line_count = contents.lines().count();
    assert_eq!(common_items.len(), line_count);

    println!("Common items:");
    let mut sum = 0;
    for c in common_items {
        print!("{} ", c);
        sum += get_priority(c);
    }
    println!("Sum: {}", sum);

    //Part Two
    let mut badges = vec!();
    let mut iter = contents.lines();
    
    assert!(line_count % 3 == 0);

    while let Some(line) = iter.next() {
        let first = line;
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                badges.push(c);
                break;
            }
        }
    }

    println!("Group badges:");
    let mut sum = 0;
    for b in badges {
        print!("{} ", b);
        sum += get_priority(b);
    }
    println!("Sum: {}", sum);
}