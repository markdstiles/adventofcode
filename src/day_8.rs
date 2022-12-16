fn get_score(row: usize, col: usize, this_value: u8, map: &Vec<u8>, width: usize, height: usize) -> usize {

    let mut left_score = 0;
    let mut right_score = 0;
    let mut up_score = 0;
    let mut down_score = 0;
    

    for x in (0..col).rev() {
        let i = (row * width) + x;

        left_score += 1;

        if map[i] >= this_value {
            break;
        }
    }

    for x in col+1..width {
        let i = (row * width) + x;

        right_score += 1;

        if map[i] >= this_value {
            break;
        }
    }

    for y in (0..row).rev() {
        let i = (y * width) + col;

        up_score += 1;

        if map[i] >= this_value {
            break;
        }
    }

    for y in row+1..height {
        let i = (y * width) + col;

        down_score += 1;

        if map[i] >= this_value {
            break;
        }
    }

    left_score * right_score * up_score * down_score
}

fn main() {
    let input = std::fs::read_to_string("day_8 input.txt").unwrap();

    let map: Vec<u8> = input.replace('\n', "").as_bytes()
            .iter()
            .map(|&x| x - 48)
            .collect();

    let width = input.lines().take(1).collect::<String>().chars().count();
    let height = input.lines().count();
    let size = width * height as usize;

    let mut vis = vec![false; size];

    //First pass rows
    for row in 0..height {
        let mut l_tallest = 0;
        let mut r_tallest = 0;

        for col in 0..width {
            //Scan left to right
            let l_scan = (row * width) + col;
            //Scan right to left
            let r_scan = (row * width) + width-1 - col;

            if row == 0 || row == height-1 || col == 0 || col == width-1 {
                vis[l_scan] = true;  //Border visibility
                vis[r_scan] = true;
            }
            else {
                if map[l_scan] > l_tallest {
                    vis[l_scan] |= true;
                }
                if map[r_scan] > r_tallest {
                    vis[r_scan] |= true;
                }
            }

            l_tallest = map[l_scan].max(l_tallest);
            r_tallest = map[r_scan].max(r_tallest);
        }
    }

    //Second pass columns
    for col in 0..width {
        let mut t_tallest = 0;
        let mut b_tallest = 0;

        for row in 0..height {
            //Scan top to bottom
            let t_scan = (row * width) + col;
            //Scan bottom to top
            let b_scan = ((height-1 - row) * width) + col;

            if row == 0 || row == height-1 || col == 0 || col == width-1 {
                vis[t_scan] = true;  //Border visibility
                vis[b_scan] = true;
            }
            else {
                if map[t_scan] > t_tallest {
                    vis[t_scan] |= true; //OR preserve visbility from previous passes
                }

                if map[b_scan] > b_tallest {
                    vis[b_scan] |= true; //OR preserve visbility from previous passes
                }
            }

            t_tallest = map[t_scan].max(t_tallest);
            b_tallest = map[b_scan].max(b_tallest);
        }
    }

    let mut scores = vec!();

    for row in 0..height {
        for col in 0..width {
            let index = (row * width) + col;
            if vis[index] {
                //print!(">{}", map[index]);
                print!("T");
            } else {
                print!(" ");
            }
            scores.push(get_score(row, col, map[index], &map, width, height));
        }
        println!();
    }

    println!("Total visible: {}", vis.iter().filter(|&x| *x).count());
    println!("Highest scenic score: {}", scores.iter().max().unwrap());
}