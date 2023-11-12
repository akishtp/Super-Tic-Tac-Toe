use std::io;
use std::fs;
use rand::Rng;

fn main() {
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

    menu(&mut game_type, &mut player);
    select_table(&mut selected_table, table, curr_player);
 
    loop{
        if game_over == true{
            break;
        }
        if game_type != 1 && player != curr_player{
            if table[selected_table - 1][1] == '─'{
                random_select_table(table, &mut selected_table);
            }
            random_bot_play(&mut selected_table, &mut curr_player, &mut table);
        }else{
            if table[selected_table - 1][1] == '─'{
                select_table(&mut selected_table, table, curr_player);
            }
            play(&mut selected_table, &mut curr_player, &mut table);
        }
        game_over = check_game(&mut selected_table, &mut table, curr_player);
    }
}

fn random_select_table(table: [[char;9];9], selected_table: &mut usize) {
    let playable_table: Vec<usize> = table.iter().enumerate()
        .filter(|(_, inner_array)| inner_array[1] != '─')
        .map(|(i, _)| i)
        .collect();
    println!("bot is selecting table from {:?}", playable_table);
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..playable_table.len());
    let number = playable_table[random_index];
    println!("bot is choose {}", number);
    
    
    *selected_table = number;
    
}

fn random_bot_play(selected_table: &mut usize, curr_player: &mut char, table: &mut [[char;9];9]) {
    let empty_positions: Vec<usize> = table[*selected_table - 1].iter().enumerate()
        .filter(|&(_, &c)| c == ' ')
        .map(|(i, _)| i)
        .collect();
    println!("bot is may play at {:?}", empty_positions);
    
    let mut rng = rand::thread_rng();
      
    loop {
        let random_index = rng.gen_range(0..empty_positions.len());

        let number = empty_positions[random_index];
        println!("bot will play at {}", number);

        if table[*selected_table - 1][number] != ' '{
            continue;
        }

        table[*selected_table - 1][number] = *curr_player;
        check_table(*selected_table, table, *curr_player);

        *selected_table = number + 1;
        if *curr_player == 'x' {
            *curr_player = 'o';
        } else {
            *curr_player = 'x';
        }
        break;
    }
}

fn menu(game_type: &mut i8, player: &mut char) {
    let ascii:String = fs::read_to_string("./img/img.txt")
        .expect("you sure you copied all my files?");
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", ascii);
    println!("1) Pass and Play\n2) Random Bot \n");
    println!("How do you want to play?");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("could not readline");
        match input.trim().parse(){
            Ok(number) => {
                if number == 1 || number == 2 {
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
    if *game_type != 1 {
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
}

fn play(selected_table : &mut usize, curr_player: &mut char, table: &mut [[char;9];9]) {
    clearscreen::clear().expect("failed to clear screen");
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

fn check_table(selected_table: usize, table: &mut [[char;9];9], curr_player: char){
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
    }
}

fn check_game(selected_table: &mut usize, table: &mut [[char;9];9], curr_player: char) -> bool{
    if (table[0][1] == '─' && table[1][1] == '─' && table[2][1] == '─' && table[0][4] == table[1][4] && table[0][4] == table[2][4]) ||
    (table[3][1] == '─' && table[4][1] == '─' && table[5][1] == '─' && table[3][4] == table[4][4] && table[3][4] == table[5][4]) ||
    (table[6][1] == '─' && table[7][1] == '─' && table[8][1] == '─' && table[6][4] == table[7][4] && table[6][4] == table[8][4]) ||
    (table[0][1] == '─' && table[3][1] == '─' && table[6][1] == '─' && table[0][4] == table[3][4] && table[0][4] == table[6][4]) ||
    (table[1][1] == '─' && table[4][1] == '─' && table[7][1] == '─' && table[1][4] == table[4][4] && table[1][4] == table[7][4]) ||
    (table[2][1] == '─' && table[5][1] == '─' && table[8][1] == '─' && table[2][4] == table[5][4] && table[2][4] == table[8][4]) ||
    (table[0][1] == '─' && table[4][1] == '─' && table[8][1] == '─' && table[0][4] == table[4][4] && table[0][4] == table[8][4]) ||
    (table[2][1] == '─' && table[4][1] == '─' && table[6][1] == '─' && table[2][4] == table[4][4] && table[2][4] == table[6][4]) {
        clearscreen::clear().expect("failed to clear screen");
        *selected_table = 10;
        if curr_player == 'x'{
            println!("\x1B[1m\x1B[4m** o Won **\x1B[24m\x1B[0m\n");
        }
        else{
            println!("\x1B[1m\x1B[4m** x Won **\x1B[24m\x1B[0m\n");
        }
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
    clearscreen::clear().expect("failed to clear screen");
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

