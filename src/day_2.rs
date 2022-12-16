use std::fs;

fn get_score(player: u8, opponent: u8) -> (u8, u8) {
    let mut player_score = player;
    let mut opponent_score = opponent;

    if player == opponent {
        //Draw
        return (player_score + 3, opponent_score + 3)
    }

    if (player > opponent                   //Scissors beat Paper, Paper beats Rock
        && !(player == 3 && opponent == 1)) //But not when player has scissors when opponent has rock
        || (player == 1 && opponent == 3)   //Or player has rock and opponent has scissors
         { 
        //Player wins
        player_score += 6;
    } else {
        //Opponent wins
        opponent_score += 6;
    }
    
    (player_score, opponent_score)
}

fn get_score_part_two(player: u8, opponent: u8) -> (u8, u8) {
    let mut player_score = 0;
    let mut opponent_score = opponent;

    //Player score now represents whether they should win, lose or draw
    //1 = lose
    //2 = draw
    //3 = win

    if player == 2 {
        //Draw - we match whatever our opponent had
        return (opponent_score + 3, opponent_score + 3)
    }

    match opponent {
        1 => {
            if player == 1 {
                /* lose */
                opponent_score += 6;
                player_score = 3;
            } else {
                /* win */
                player_score = 8; // 2 + 6
            }
        },
        2 => {
            if player == 1 {
                /* lose */
                opponent_score += 6;
                player_score = 1;
            } else {
                /* win */
                player_score = 9; // 3 + 6
            }
        },
        3 => {
            if player == 1 {
                /* lose */
                opponent_score += 6;
                player_score = 2;
            } else {
                /* win */
                player_score = 7; // 1 + 6
            }
        },
        _ => ( /* will never happen */)
    }

    (player_score, opponent_score)
}

fn main() {
    //Rock Paper Scissors
    //Rock = 1 (A or X)
    //Paper = 2 (B or Y)
    //Scissors = 3 (C or Z)

    let contents = fs::read_to_string("day_2 input.txt").unwrap();

    let mut my_total_score: u32 = 0;
    let mut opp_total_score: u32 = 0;

    for line in contents.lines() {
        let opponent = line.chars().next().unwrap() as u8 - 64;
        let player = line.chars().nth(2).unwrap() as u8 - 87;

        let (my_score, opp_score) = get_score_part_two(player, opponent);
        
        my_total_score += my_score as u32;
        opp_total_score += opp_score as u32;
    }

    println!("My score {}, Opponent score {}", my_total_score, opp_total_score);
}