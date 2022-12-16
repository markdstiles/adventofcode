fn main() {
    let input = std::fs::read_to_string("Day_9 input.txt").unwrap();

    let knots = 9;
    let mut knot_coords = vec!();
    let mut knot_pos = vec!();
    let mut h_coords = vec!();
    
    let mut h_pos: (i32, i32) = (0, 0); //Head position
    h_coords.push(h_pos);               //Record Head starting positon

    for knot in 0..knots {
        knot_coords.push(vec!());
        knot_pos.push(h_pos);    //Knot position
        knot_coords[knot].push(h_pos); //Record starting positions of each knot
    }    

    for line in input.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        let dir = command[0];
        let amount: isize = command[1].parse().unwrap();

        //Move the head
        for _ in 0..amount {
            match dir {
                "U" => h_pos.1 += -1,
                "R" => h_pos.0 += 1,
                "D" => h_pos.1 += 1,
                "L" => h_pos.0 += -1,
                _ => (),
            }

            //Update knot positions
            let mut prev_knot = h_pos;
           
            for knot in 0..knots {
                let k_pos = knot_pos[knot];

                if knot != 0 {  //Second knot follows head, otherwise follow previous knot
                    prev_knot = knot_pos[knot-1];
                }
                
                let diff = (k_pos.0 - prev_knot.0, k_pos.1 - prev_knot.1);

                let apply = match diff {
                    (2, 2)      => (1, 1),
                    (-2, 2)     => (-1, 1),
                    (2, -2)     => (1, -1),
                    (-2, -2)    => (-1, -1),
                    (2, _)      => (1, 0),
                    (_, 2)      => (0, 1),
                    (-2, _)     => (-1, 0),
                    (_, -2)     => (0, -1),
                    _           => (0, 0)
                };
                
                if apply != (0,0) {
                    knot_pos[knot].0 = prev_knot.0 + apply.0;
                    knot_pos[knot].1 = prev_knot.1 + apply.1;
                    knot_coords[knot].push(knot_pos[knot]);
                }
            }

            h_coords.push(h_pos);
        }
    }

    let t_coords = knot_coords.last().unwrap();
    let x_min = t_coords.iter().map(|c| c.0).min().unwrap();
    let y_min = t_coords.iter().map(|c| c.1).min().unwrap();
    let width = t_coords.iter().map(|c| c.0).max().unwrap() - x_min;
    let height = t_coords.iter().map(|c| c.1).max().unwrap() - y_min;
    let size = (width+1) * (height+1);

    let mut map = vec![false; size as usize];
    for coord in t_coords.iter() {
        let x = x_min.abs() + coord.0;
        let y = y_min.abs() + coord.1;
        let index = ((y * (width+1)) + x) as usize;
        map[index] = true;
    }

    let start_x = x_min.abs() + t_coords[0].0;
    let start_y = y_min.abs() + t_coords[0].1;

    for row in 0..height+1 {
        for col in 0..width+1 {
            let index = ((row * (width+1)) + col) as usize;
            if col == start_x && row == start_y {
                print!("s");
            } else if map[index] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("Visits: {}", map.iter().filter(|&x| *x).count());
}