use std::io;
use std::fs;
use rand::Rng;

fn main() {
    let mut quit_program: bool = false;
    while quit_program == false{
        let mut table:[[char;9];9] = [
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        ];
        let mut selected_table: usize = 10;
        let mut curr_player: char = 'x';
        let mut game_over: bool = false;
        let mut game_type: i8 = 0;
        let mut player: char = ' ';

        menu(&mut game_type, &mut player, &mut quit_program);

        if game_type == 1 || player == curr_player {
            select_table(&mut selected_table, table, curr_player);
        } else {
            random_select_table(table, &mut selected_table);
        }

        loop{
            if game_over == true{
                loop {
                    let mut input = String::new();
                    if game_type == 0 {
                        println!("The bots had a fun time playing without you\nLast chance to join..\n(y = join / n = leave)");
                    }
                    else{
                        println!("Wanna play again? (y/n)");
                    }
                    io::stdin()
                        .read_line(&mut input)
                        .expect("could not readline");
                    input = input.trim().to_string();
                    if input == "y" || input == "n"{
                        if input == "y" { quit_program = false; }
                        else if input == "n" { quit_program = true; }
                    }
                    else{
                        println!("Choose either y or n");
                        continue;
                    }
                    break;
                }
                break;
            }
            if game_type != 1 && player != curr_player{
                if game_type == 2 || game_type == 0 {
                    if table[selected_table - 1][1] == '─'{
                        random_select_table(table, &mut selected_table);
                    }
                    random_bot_play(&mut selected_table, &mut curr_player, &mut table);
                } else if game_type == 3 {
                    if table[selected_table - 1][1] == '─'{
                        random_select_table(table, &mut selected_table);
                    }
                    minmax_bot_play(&mut selected_table, &mut curr_player, &mut table);
                }
            }else{
                if table[selected_table - 1][1] == '─'{
                    select_table(&mut selected_table, table, curr_player);
                }
                play(&mut selected_table, &mut curr_player, &mut table);
            }
            game_over = check_game(&mut selected_table, &mut table, curr_player);
        }
        if quit_program == false {
            continue;
        }
        break;
    }
}

fn random_select_table(table: [[char;9];9], selected_table: &mut usize) {
    let playable_table: Vec<usize> = table.iter().enumerate()
        .filter(|(_, inner_array)| inner_array[1] != '─')
        .map(|(i, _)| i)
        .collect();
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..playable_table.len());
    let number = playable_table[random_index];

    *selected_table = number + 1;
}

fn random_bot_play(selected_table: &mut usize, curr_player: &mut char, table: &mut [[char;9];9]) {
    // add an interval of like 1sec or sm
    let empty_positions: Vec<usize> = table[*selected_table - 1].iter().enumerate()
        .filter(|&(_, &c)| c == ' ')
        .map(|(i, _)| i)
        .collect();

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..empty_positions.len());
    let number = empty_positions[random_index];
    table[*selected_table - 1][number] = *curr_player;
    check_table(*selected_table, table, *curr_player);

    *selected_table = number + 1;
    if *curr_player == 'x' {
        *curr_player = 'o';
    } else {
        *curr_player = 'x';
    }
}

fn minmax_bot_play(selected_table: &mut usize, curr_player: &mut char, table: &mut [[char;9];9]) {
    let empty_positions: Vec<usize> = table[*selected_table - 1].iter().enumerate()
        .filter(|&(_, &c)| c == ' ')
        .map(|(i, _)| i)
        .collect();

    let mut best_move: i32 = -1;
    let mut best_score: i32 = -10;

    for i in 0..(empty_positions.len()) {
        let score: i32 = minimax(table[*selected_table - 1], 0, false, *curr_player);

        if score > best_score {
            println!("score{} best_score{}", score, best_score);
            best_score = score;
            best_move = i as i32;
        }
    }
    let number = empty_positions[best_move as usize];
    table[*selected_table - 1][number] = *curr_player;
    check_table(*selected_table, table, *curr_player);

    *selected_table = number + 1;
    if *curr_player == 'x' {
        *curr_player = 'o';
    } else {
        *curr_player = 'x';
    }
}

fn minimax(board: [char;9], depth: u8, is_maximizing: bool, curr_player: char) -> i32 {
    let result: isize = just_check_table(board, curr_player);
    if result != 10 {
        return result as i32;
    }
    if is_maximizing {
        
    } else {
        //
    }
    0
}

fn menu(game_type: &mut i8, player: &mut char, quit_program: &mut bool) {
    let ascii:String = fs::read_to_string("./img/img.txt")
        .expect("You sure you copied all my files?");
    //clearscreen::clear().expect("failed to clear screen");
    println!("{}", ascii);
    println!("1) Pass and Play\n2) Random Bot \n3) Minimax Bot\n\n0) quit\n");
    println!("How do you want to play?");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not readline");
        match input.trim().parse(){
            Ok(number) => {
                if number == 1 || number == 2 || number == 3 || number == 0 {
                    *game_type = number;
                    break;
                }
                else{
                    eprintln!("Select from one of the options");
                    continue;
                }
            }
            Err(err) => {
                eprintln!("error: {}",err);
            }
        }
    }
    if *game_type == 2 || *game_type == 3 {
        println!("Which player are you? \x1B[24m(x or o)\x1B[0m");
        loop{
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("could not readline");
            match input.trim().parse::<char>() {
                Ok(charecter) => {
                    if charecter == 'x' || charecter == 'o'{
                        *player = charecter;
                        break;
                    }
                    else{
                        println!("Choose x or o");
                        continue
                    }
                }
                Err(err) => {
                    eprintln!("Error : {}", err);
                }
            }
        }
    }
    if *game_type == 0 {
        *quit_program = true;
    }
}

fn play(selected_table : &mut usize, curr_player: &mut char, table: &mut [[char;9];9]) {
    //clearscreen::clear().expect("failed to clear screen");
    println!("\x1B[1m\x1B[4m** PLAY GAME **\x1B[24m\x1B[0m\n");
    draw_table(*selected_table, *table);
    println!("Player\x1B[1m {}\x1B[0m's turn \x1B[1m(1-9)\x1B[0m:", curr_player);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        match input.trim().parse::<usize>() {
            Ok(number) => {

                if !(1..=9).contains(&number){
                    println!("Select a number between 1 and 9");
                    continue;
                }
                if table[*selected_table - 1][number - 1] != ' '{
                    println!("Please select an empty space");
                    continue;
                }

                table[*selected_table - 1][number - 1] = *curr_player;
                
                check_table(*selected_table, table, *curr_player);

                *selected_table = number;
                

                if *curr_player == 'x' {
                    *curr_player = 'o';
                } else {
                    *curr_player = 'x';
                }

                break;
            }
            Err(_) => {
                eprintln!("Select a number between 1 and 9");
                continue;
            }
        }
    }

}

fn just_check_table(board: [char;9], curr_player: char) -> isize {
    let empty_positions: Vec<usize> = board.iter().enumerate()
        .filter(|&(_, &c)| c == ' ')
        .map(|(i, _)| i)
        .collect();
    if board[0] != ' ' && board[0] == board[1] && board[0] == board[2] {if curr_player == board[0] { return 1; } else { return -1; }}
    else if board[3] != ' ' && board[3] == board[4] && board[3] == board[5] {if curr_player == board[3] { return 1; } else { return -1; }}
    else if board[6] != ' ' && board[6] == board[7] && board[6] == board[8] {if curr_player == board[6] { return 1; } else { return -1; }}
    else if board[0] != ' ' && board[0] == board[3] && board[0] == board[6] {if curr_player == board[0] { return 1; } else { return -1; }}
    else if board[1] != ' ' && board[1] == board[4] && board[1] == board[7] {if curr_player == board[1] { return 1; } else { return -1; }}
    else if board[2] != ' ' && board[2] == board[5] && board[2] == board[8] {if curr_player == board[2] { return 1; } else { return -1; }}
    else if board[0] != ' ' && board[0] == board[4] && board[0] == board[8] {if curr_player == board[0] { return 1; } else { return -1; }}
    else if board[2] != ' ' && board[2] == board[4] && board[2] == board[6] {if curr_player == board[2] { return 1; } else { return -1; }}
    else if empty_positions.is_empty() { return 0; }
    10
}

fn check_table(selected_table: usize, table: &mut [[char;9];9], curr_player: char){
    let empty_positions: Vec<usize> = table[selected_table - 1].iter().enumerate()
        .filter(|&(_, &c)| c == ' ')
        .map(|(i, _)| i)
        .collect();
    if (table[selected_table - 1][0] == curr_player && table[selected_table - 1][1] == curr_player && table[selected_table - 1][2] == curr_player)||
    (table[selected_table - 1][3] == curr_player && table[selected_table - 1][4] == curr_player && table[selected_table - 1][5] == curr_player) ||
    (table[selected_table - 1][6] == curr_player && table[selected_table - 1][7] == curr_player && table[selected_table - 1][8] == curr_player) ||
    (table[selected_table - 1][0] == curr_player && table[selected_table - 1][3] == curr_player && table[selected_table - 1][6] == curr_player) ||
    (table[selected_table - 1][1] == curr_player && table[selected_table - 1][4] == curr_player && table[selected_table - 1][7] == curr_player) ||
    (table[selected_table - 1][2] == curr_player && table[selected_table - 1][5] == curr_player && table[selected_table - 1][8] == curr_player) ||
    (table[selected_table - 1][0] == curr_player && table[selected_table - 1][4] == curr_player && table[selected_table - 1][8] == curr_player) ||
    (table[selected_table - 1][2] == curr_player && table[selected_table - 1][4] == curr_player && table[selected_table - 1][6] == curr_player) {
        table[selected_table - 1][0] = '┌';
        table[selected_table - 1][1] = '─';
        table[selected_table - 1][2] = '┐';
        table[selected_table - 1][3] = '│';
        table[selected_table - 1][4] = curr_player;
        table[selected_table - 1][5] = '│';
        table[selected_table - 1][6] = '└';
        table[selected_table - 1][7] = '─';
        table[selected_table - 1][8] = '┘';
    } else if empty_positions.is_empty() {
        table[selected_table - 1][0] = '┌';
        table[selected_table - 1][1] = '─';
        table[selected_table - 1][2] = '┐';
        table[selected_table - 1][3] = '│';
        table[selected_table - 1][4] = ' ';
        table[selected_table - 1][5] = '│';
        table[selected_table - 1][6] = '└';
        table[selected_table - 1][7] = '─';
        table[selected_table - 1][8] = '┘';
    }
}

fn check_game(selected_table: &mut usize, table: &mut [[char;9];9], curr_player: char) -> bool{
    let playable_table: Vec<usize> = table.iter().enumerate()
        .filter(|(_, inner_array)| inner_array[1] != '─')
        .map(|(i, _)| i)
        .collect();
    //clearscreen::clear().expect("failed to clear screen");

    if (table[0][1] == '─' && table[1][1] == '─' && table[2][1] == '─' && table[0][4] == table[1][4] && table[0][4] == table[2][4]  ) ||
    (table[3][1] == '─' && table[4][1] == '─' && table[5][1] == '─' && table[3][4] == table[4][4] && table[3][4] == table[5][4]) ||
    (table[6][1] == '─' && table[7][1] == '─' && table[8][1] == '─' && table[6][4] == table[7][4] && table[6][4] == table[8][4]) ||
    (table[0][1] == '─' && table[3][1] == '─' && table[6][1] == '─' && table[0][4] == table[3][4] && table[0][4] == table[6][4]) ||
    (table[1][1] == '─' && table[4][1] == '─' && table[7][1] == '─' && table[1][4] == table[4][4] && table[1][4] == table[7][4]) ||
    (table[2][1] == '─' && table[5][1] == '─' && table[8][1] == '─' && table[2][4] == table[5][4] && table[2][4] == table[8][4]) ||
    (table[0][1] == '─' && table[4][1] == '─' && table[8][1] == '─' && table[0][4] == table[4][4] && table[0][4] == table[8][4]) ||
    (table[2][1] == '─' && table[4][1] == '─' && table[6][1] == '─' && table[2][4] == table[4][4] && table[2][4] == table[6][4]) {
        *selected_table = 10;
        if curr_player == 'x'{
            println!("\x1B[1m\x1B[4m** o WON **\x1B[24m\x1B[0m\n");
        }
        else{
            println!("\x1B[1m\x1B[4m** x WON **\x1B[24m\x1B[0m\n");
        }
        draw_table(*selected_table, *table);
        return true;
    } else if playable_table.is_empty(){
        *selected_table = 10;
        println!("\x1B[1m\x1B[4m** DRAW GAME **\x1B[24m\x1B[0m\n");
        draw_table(*selected_table, *table);
        return true;
    }
    false
}

fn draw_table(selected_table: usize, table: [[char;9];9]) {
    println!("┌─┬─────┬─┬─────┬─┬─────┬─┐");


    if selected_table == 1{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][0], table[0][1], table[0][2], table[1][0], table[1][1], table[1][2], table[2][0], table[2][1], table[2][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][3], table[0][4], table[0][5], table[1][3], table[1][4], table[1][5], table[2][3], table[2][4], table[2][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][6], table[0][7], table[0][8], table[1][6], table[1][7], table[1][8], table[2][6], table[2][7], table[2][8]);
    }else if selected_table == 2{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[0][0], table[0][1], table[0][2], table[1][0], table[1][1], table[1][2], table[2][0], table[2][1], table[2][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[0][3], table[0][4], table[0][5], table[1][3], table[1][4], table[1][5], table[2][3], table[2][4], table[2][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[0][6], table[0][7], table[0][8], table[1][6], table[1][7], table[1][8], table[2][6], table[2][7], table[2][8]);
    }else if selected_table == 3{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[0][0], table[0][1], table[0][2], table[1][0], table[1][1], table[1][2], table[2][0], table[2][1], table[2][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[0][3], table[0][4], table[0][5], table[1][3], table[1][4], table[1][5], table[2][3], table[2][4], table[2][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[0][6], table[0][7], table[0][8], table[1][6], table[1][7], table[1][8], table[2][6], table[2][7], table[2][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][0], table[0][1], table[0][2], table[1][0], table[1][1], table[1][2], table[2][0], table[2][1], table[2][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][3], table[0][4], table[0][5], table[1][3], table[1][4], table[1][5], table[2][3], table[2][4], table[2][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[0][6], table[0][7], table[0][8], table[1][6], table[1][7], table[1][8], table[2][6], table[2][7], table[2][8]);
    }


    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");


    if selected_table == 4{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][0], table[3][1], table[3][2], table[4][0], table[4][1], table[4][2], table[5][0], table[5][1], table[5][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][3], table[3][4], table[3][5], table[4][3], table[4][4], table[4][5], table[5][3], table[5][4], table[5][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][6], table[3][7], table[3][8], table[4][6], table[4][7], table[4][8], table[5][6], table[5][7], table[5][8]);
    }else if selected_table == 5{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[3][0], table[3][1], table[3][2], table[4][0], table[4][1], table[4][2], table[5][0], table[5][1], table[5][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[3][3], table[3][4], table[3][5], table[4][3], table[4][4], table[4][5], table[5][3], table[5][4], table[5][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[3][6], table[3][7], table[3][8], table[4][6], table[4][7], table[4][8], table[5][6], table[5][7], table[5][8]);
    }else if selected_table == 6{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[3][0], table[3][1], table[3][2], table[4][0], table[4][1], table[4][2], table[5][0], table[5][1], table[5][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[3][3], table[3][4], table[3][5], table[4][3], table[4][4], table[4][5], table[5][3], table[5][4], table[5][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[3][6], table[3][7], table[3][8], table[4][6], table[4][7], table[4][8], table[5][6], table[5][7], table[5][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][0], table[3][1], table[3][2], table[4][0], table[4][1], table[4][2], table[5][0], table[5][1], table[5][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][3], table[3][4], table[3][5], table[4][3], table[4][4], table[4][5], table[5][3], table[5][4], table[5][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[3][6], table[3][7], table[3][8], table[4][6], table[4][7], table[4][8], table[5][6], table[5][7], table[5][8]);
    }


    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");


    if selected_table == 7{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][0], table[6][1], table[6][2], table[7][0], table[7][1], table[7][2], table[8][0], table[8][1], table[8][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][3], table[6][4], table[6][5], table[7][3], table[7][4], table[7][5], table[8][3], table[8][4], table[8][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][6], table[6][7], table[6][8], table[7][6], table[7][7], table[7][8], table[8][6], table[8][7], table[8][8]);
    }else if selected_table == 8{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[6][0], table[6][1], table[6][2], table[7][0], table[7][1], table[7][2], table[8][0], table[8][1], table[8][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[6][3], table[6][4], table[6][5], table[7][3], table[7][4], table[7][5], table[8][3], table[8][4], table[8][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", table[6][6], table[6][7], table[6][8], table[7][6], table[7][7], table[7][8], table[8][6], table[8][7], table[8][8]);
    }else if selected_table == 9{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[6][0], table[6][1], table[6][2], table[7][0], table[7][1], table[7][2], table[8][0], table[8][1], table[8][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[6][3], table[6][4], table[6][5], table[7][3], table[7][4], table[7][5], table[8][3], table[8][4], table[8][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", table[6][6], table[6][7], table[6][8], table[7][6], table[7][7], table[7][8], table[8][6], table[8][7], table[8][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][0], table[6][1], table[6][2], table[7][0], table[7][1], table[7][2], table[8][0], table[8][1], table[8][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][3], table[6][4], table[6][5], table[7][3], table[7][4], table[7][5], table[8][3], table[8][4], table[8][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", table[6][6], table[6][7], table[6][8], table[7][6], table[7][7], table[7][8], table[8][6], table[8][7], table[8][8]);
    }


    println!("└─┴─────┴─┴─────┴─┴─────┴─┘");
}

fn select_table(selected_table: &mut usize, table: [[char;9];9], curr_player: char){
    //clearscreen::clear().expect("failed to clear screen");
    *selected_table = 10;
    println!("\x1B[1m\x1B[4m** SELECT A TABLE **\x1B[24m\x1B[0m\n");
    draw_table(*selected_table, table);
    println!("Which box do you want to play, \x1B[1m{}\x1B[0m? \x1B[1m(1-9)\x1B[0m", curr_player);
    
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
    
        match input.trim().parse() {
            Ok(number) => {
                *selected_table = number;

                if !(1..=9).contains(selected_table){
                    println!("Select a number between 1 and 9");
                    continue;
                }
                if table[*selected_table - 1][1] == '─' {
                    println!("Select a playable table");
                    continue;
                }
                break;
            }
            Err(_) => {
                eprintln!("Select a number between 1 and 9");
                continue;
            }
        }
    }
}

