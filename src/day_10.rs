fn main() {
    let input = std::fs::read_to_string("day_10 input.txt").unwrap();

    let mut values = vec![1];

    for line in input.lines() {
        if line == "noop" {
            values.push(0);
        } else {
            let value = line.replace("addx ", "").parse().unwrap();
            values.push(0);
            values.push(value);
        }
    }

    let at20 = values.iter().take(20).sum::<isize>() * 20;
    let at60 = values.iter().take(60).sum::<isize>() * 60;
    let at100 = values.iter().take(100).sum::<isize>() * 100;
    let at140 = values.iter().take(140).sum::<isize>() * 140;
    let at180 = values.iter().take(180).sum::<isize>() * 180;
    let at220 = values.iter().take(220).sum::<isize>() * 220;

    println!("Total: {}", at20 + at60 + at100 + at140 + at180 + at220);

    let mut sprite_pos = 0;
    for (crt_pos, value) in values.iter().enumerate().take(240) {
        sprite_pos += value;

        let crt_2_spr = (crt_pos as isize) % 40;
        if crt_2_spr >= sprite_pos - 1 && crt_2_spr <= sprite_pos + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if (crt_pos+1) % 40 == 0 && crt_pos != 0 {
            println!();
        }
    }
}