use std::{array, clone, ops, fmt::format, io, ptr::null};


#[derive(Debug, Clone, Copy)]
enum TicTacType {
    Cross,
    Circle
}

#[derive(Debug, Clone, Copy)]
struct Square {
    input_type: TicTacType,
    is_occupied: bool,
}

impl Square {
    fn new() -> Self {
        Self {
            input_type: TicTacType::Cross,
            is_occupied: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    tic_tac_type: Option<TicTacType>,
    score: Vec<u8>
}

impl Player {
    fn new() -> Self {
        Self {
            name: String::new(),
            tic_tac_type: None,
            score: vec![]
        }
    }

    fn name(mut self, player: String) -> Self {
        self.name = player;
        self
    }

    fn tic_tac_type(mut self, selection: TicTacType) -> Self {
        self.tic_tac_type = Some(selection);
        self
    }

    fn score(&mut self, input: u8) -> () {
        self.score.push(input);
    }
}

fn input_player_name(num: u8) -> Player {
    let mut tic_tac_type: String = String::new();
    let mut player_name: String = String::new();

    loop {
        println!("Please enter name for player {}", num);
        let result = io::stdin().read_line(&mut player_name);

        if result.is_err() {
            println!("Input failed!, input for player {} again", num);
            continue
        }


        if player_name.trim().is_empty() {
            println!("Input failed!, input for player {} cannot be blank!", num);
            println!("Please try again!");
            println!("");
            continue
        }

        if !player_name.trim().is_empty() {
            print!("Player {} is now {}", num, player_name);
            break;
        }
    }

    loop {
        println!("Please either choose cross (x) or circle (o)");
        let result: Result<usize, io::Error> = io::stdin().read_line(&mut tic_tac_type);

        if result.is_err() {
            println!("Input failed!, choose x or o only");
            continue
        }

        if tic_tac_type.trim().is_empty() {
            println!("Input failed!, input for player cannot be blank!");
            println!("Please try again!");
            println!("");
            continue
        }

        


        if tic_tac_type.trim() != "x" && tic_tac_type.trim() != "o" {
            println!("Input failed!, input {} can only be 'o' or 'x'!", tic_tac_type);
            println!("Please try again!");
            println!("");
            continue
        }

        break;
    }

    println!("check player {player_name}, player choice {tic_tac_type}");
    if tic_tac_type.trim().to_string() == "x" {
        println!("cross_select");
        Player::new().name(player_name.trim().to_string()).tic_tac_type(TicTacType::Cross)
    } else {
        Player::new().name(player_name).tic_tac_type(TicTacType::Circle)
    }
}


fn check_win_condition(player: &Player) -> bool {
    // step 1 check score
    // combination check
    const CONDITION_1: [u8; 3] = [0, 1, 2];
    const CONDITION_2: [u8; 3] = [3, 4, 5];
    const CONDITION_3: [u8; 3] = [6, 7, 8];
    const CONDITION_4: [u8; 3] = [0, 3, 6];
    const CONDITION_5: [u8; 3] = [1, 4, 7];
    const CONDITION_6: [u8; 3] = [2, 5, 8];
    const CONDITION_7: [u8; 3] = [0, 4, 8];
    const CONDITION_8: [u8; 3] = [2, 4, 6];

    let player_score: &Vec<u8> = &player.score;

    const IS_MATCH: bool = false;

    if CONDITION_1.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_2.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_3.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_4.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_5.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_6.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_7.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    if CONDITION_8.iter().all(|&x| player_score.contains(&x)) {
        return true;
    }

    IS_MATCH
}

fn generate_score_board(board_state: &Vec<Square>) -> () {
    
    println!("generating score board now...");

    const ROW_1: [i32; 3] = [0, 1, 2];
    const ROW_2: [i32; 3] = [3, 4, 5];
    const ROW_3: [i32; 3] = [6, 7, 8];

    let mut row_1_display: String = String::new();
    let mut row_2_display: String = String::new();
    let mut row_3_display: String = String::new();

    for (_i, val) in ROW_1.iter().enumerate() {
        if board_state[*val as usize].is_occupied {
            match board_state[*val as usize].input_type {
                TicTacType::Cross => row_1_display.push_str(" [ X ] "),
                TicTacType::Circle => row_1_display.push_str(" [ O ] "),
            }
        } else {
            row_1_display.push_str(" [   ] ");
        }
    }
    for (_i, val) in ROW_2.iter().enumerate() {
        if board_state[*val as usize].is_occupied {
            match board_state[*val as usize].input_type {
                TicTacType::Cross => row_2_display.push_str(" [ X ] "),
                TicTacType::Circle => row_2_display.push_str(" [ O ] "),
            }
        } else {
            row_2_display.push_str(" [   ] ");
        }
    }
    for (_i, val) in ROW_3.iter().enumerate() {
        if board_state[*val as usize].is_occupied {
            match board_state[*val as usize].input_type {
                TicTacType::Cross => row_3_display.push_str(" [ X ] "),
                TicTacType::Circle => row_3_display.push_str(" [ O ] "),
            }
        } else {
            row_3_display.push_str(" [   ] ");
        }
    }

    println!("{row_1_display}");
    println!("{row_2_display}");
    println!("{row_3_display}");
}


fn run_game(game_board: &mut Vec<Square>, player_1: &mut Player, player_2: &mut Player) {
    let range = 0..=8;

    println!("check player {:#?}", {&player_1});

    loop {
        let player_1_win_state = check_win_condition(&player_1);
        let player_2_win_state = check_win_condition(&player_2);

        generate_score_board(&game_board);

        if player_1_win_state {
            println!("Player 1: {} is the winner!", player_1.name);
            break
        }

        if player_2_win_state {
            println!("Player 2: {} is the winner!", player_2.name);
            break
        }

        let mut input: String = String::new();

        loop {
            println!("Player 1 input position into board");
            println!("");
            
            if let Ok(_) = io::stdin().read_line(&mut input) {
                println!("Input {}, received!", input.trim())
            } else {
                eprintln!("Error failed input")
            };

            let val_res = match input.trim().parse::<u8>() {
                Ok(val) => val,
                Err(err) => {
                    println!("Failed to parse! error {}", err);
                    continue
                }
            };
            //  update game board
            let board_sq: &Square = match game_board.get(val_res as usize) {
                Some(val) => val,
                None => {
                    println!("None!");
                    continue
                }
            };

            if board_sq.is_occupied {
                println!("Square is occupied! please select again!");
                continue
            }

            let player_1_tic_tac_type = player_1.tic_tac_type.clone();

            let tic_tac_input = match player_1_tic_tac_type {
                Some(val) => val,
                None => {
                    println!("Not assigned tic tac val for player 1!");
                    continue
                }
            };

            game_board[val_res as usize].is_occupied = true;
            game_board[val_res as usize].input_type = tic_tac_input.clone();

            //  update player score
            player_1.score(val_res);

            println!("player 1 state = {:?}", player_1.score);

            break
        }

        generate_score_board(&game_board);

        loop {
            println!("Player 2 input position into board");
            println!("");
            
            if let Ok(_) = io::stdin().read_line(&mut input) {
                println!("Input {}, received!", input.trim())
            } else {
                eprintln!("Error failed input")
            };

            println!("check P2 {input}");

            let val_res = match input.trim().parse::<u8>() {
                Ok(val) => val,
                Err(err) => {
                    println!("Failed to parse! error {}", err);
                    continue
                }
            };
            //  update game board
            let board_sq: &Square = match game_board.get(val_res as usize) {
                Some(val) => val,
                None => {
                    println!("None!");
                    continue
                }
            };

            if board_sq.is_occupied {
                println!("Square is occupied! please select again!");
                continue
            }

            let tic_tac_input = match player_2.tic_tac_type {
                Some(val) => val,
                None => {
                    println!("Not assigned tic tac val for player 1!");
                    continue
                }
            };

            game_board[val_res as usize].is_occupied = true;
            game_board[val_res as usize].input_type = tic_tac_input.clone();

            //  update player score
            player_2.score(val_res);

            // println!("player 1 state = {:?}", player_1.score);

            break
        }

        // print score board
        generate_score_board(&game_board);

        // for (index, score) in game_board.iter().enumerate() {
        //     let mut row: String = String::new();

        //     if index <= 2 {
        //         if score.is_occupied {
        //             row.push_str(" [   ] ");
                    
        //         } else {
        //             let input_type: String = match score.input_type {
        //                 TicTacType::Circle => "O".to_string(),
        //                 TicTacType::Cross => "X".to_string(), 
        //             };

        //             let s: String = format!(" [ {} ] ", input_type);
        //             row.push_str(&s);

        //         }
        //     }
        // }
    }
}

fn main() {
    println!("Running tic tac toe game");

    let new_tic_tac_item: Square = Square::new();

    // let state: [TicTac; 9] = [NEW_TIC_TAC_ITEM; 9];
    // let state: [TicTac; 9] = core::vec::from_fn(|_| new_tic_tac_item.clone());
    let mut state = vec![new_tic_tac_item.clone(); 9];

    // println!("{:?}", state);

    let mut player_1: Player = input_player_name(1);
    let mut player_2: Player = input_player_name(2);

    run_game(&mut state, &mut player_1, &mut player_2);
}
