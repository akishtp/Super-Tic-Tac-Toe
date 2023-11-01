use std::io;

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
    let mut player: char = 'x';
    select_table(&mut selected_table, table);
    play(selected_table, player, &mut table);
    draw_table(selected_table, table);
}

fn play(selected_table :usize, player: char, table: &mut [[char;9];9]) {
    draw_table(selected_table, *table);
    println!("Player\x1B[1m {}\x1B[0m's turn (1-9):", player);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        match input.trim().parse::<usize>() {
            Ok(number) => {
                table[selected_table - 1][number - 1] = player;
                break;
            }
            Err(_) => {
                eprintln!("Select a number between 1 and 9");
                continue;
            }
        }
    }
}

fn draw_table(selected_table: usize, table: [[char;9];9]) {
    clearscreen::clear().expect("failed to clear screen");
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

fn select_table(selected_table: &mut usize, table: [[char;9];9]){
    draw_table(*selected_table, table);
    println!("┌─┬─────┬─┬─────┬─┬─────┬─┐");
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("│ │ ╎1╎ │ │ ╎2╎ │ │ ╎3╎ │ │");
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("│ │ ╎4╎ │ │ ╎5╎ │ │ ╎6╎ │ │");
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("├─┼─────┼─┼─────┼─┼─────┼─┤"); 
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("│ │ ╎7╎ │ │ ╎8╎ │ │ ╎9╎ │ │");
    println!("│ │ ╎ ╎ │ │ ╎ ╎ │ │ ╎ ╎ │ │");
    println!("└─┴─────┴─┴─────┴─┴─────┴─┘");
    println!("Which box do you want to play?");
    
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
                break;
            }
            Err(_) => {
                eprintln!("Enter a smaller positive number");
                continue;
            }
        }
    }
}

