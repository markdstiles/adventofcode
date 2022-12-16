use adventofcode::FindAll;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Sqr,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    op_amount: i64,
    divisor: i64,
    true_monkey: usize,
    false_monkey: usize
}

fn main() {
    let monkey_count = 8;
    let rounds = 10_000;

    let input = std::fs::read_to_string("day_11 input.txt").unwrap();
    let mut monkeys: Vec<Monkey> = vec!();
    let mut inspections = vec![0_i64; monkey_count];

    let input = input.replace("\r\n", "\n");

    let pat = "Monkey {}:\n  Starting items: {}\n  Operation: new = old {} {}\n  Test: divisible by {}\n    If true: throw to monkey {}\n    If false: throw to monkey {}\n";

    for m in input.find_all(pat) {
        let items: Vec<i64> = m[1].split(", ").map(|x| x.parse::<i64>().unwrap()).collect();
        let (op, op_amount) = match m[2].as_str() {
            "+" => {
                let amount = m[3].parse().unwrap();
                (Op::Add, amount)
            }
            "*" => {
                if m[3] == "old" {
                    (Op::Sqr, 0)
                } else {
                    let amount = m[3].parse().unwrap();
                    (Op::Mul, amount)
                }
            },
            _ => panic!(),
        };
        let divisor = m[4].parse().unwrap();
        let true_monkey = m[5].parse().unwrap();
        let false_monkey = m[6].parse().unwrap();

        monkeys.push(Monkey {
            items,
            op,
            op_amount,
            divisor,
            true_monkey,
            false_monkey,
        });
    }

    let mod_amount = monkeys.iter()
        .map(|m| m.divisor)
        .collect::<Vec<i64>>().iter().copied()
        .reduce(|a, b| a*b).unwrap();

    for _ in 1..=rounds {
        for m in 0..monkeys.len() {
            let mut items_to_remove: Vec<usize> = vec!();
            let mut items_to_move: Vec<(usize, i64)> = vec!();

            for (j, val) in monkeys[m].items.iter().enumerate() {
                let mut item = *val;

                match monkeys[m].op {
                    Op::Add => item += monkeys[m].op_amount,
                    Op::Mul => item *= monkeys[m].op_amount,
                    Op::Sqr => item *= item,
                }

                inspections[m] += 1;

                item %= mod_amount;
                //item /= 3;
                
                if item % monkeys[m].divisor == 0 {
                    items_to_move.push((monkeys[m].true_monkey, item));
                } else {
                    items_to_move.push((monkeys[m].false_monkey, item));
                }
                items_to_remove.push(j);
            }

            for i in items_to_remove.iter().rev() {
                monkeys[m].items.remove(*i);
            }

            for (i, item) in items_to_move {
                monkeys[i].items.push(item);
            }
        }
    }

    for index in 0..monkey_count {
        println!("Monkey {} inspected items {} times.", index, inspections[index]);
    }

    inspections.sort();
    let results: Vec<i64> = inspections.iter().copied().rev().take(2).collect();
    println!("Total monkey business: {}", results[0] * results[1]);
}