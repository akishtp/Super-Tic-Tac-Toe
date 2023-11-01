use std::io;

static TABLE:[[char;9];9] = [
    [' ', ' ', ' ', ' ', '1', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '2', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '3', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '4', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '5', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '6', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '7', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '8', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', '9', ' ', ' ', ' ', ' ']
];

fn main() {
    let round:i8 = 0;
    let mut selected_table:i8 = 0;
    draw_table(selected_table);
    play(round, selected_table.to_string());
}

fn draw_table(selected_table:i8) {
//    clearscreen::clear().expect("failed to clear screen");
    println!("┌─┬─────┬─┬─────┬─┬─────┬─┐");


    if selected_table == 0{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][0], TABLE[0][1], TABLE[0][2], TABLE[1][0], TABLE[1][1], TABLE[1][2], TABLE[2][0], TABLE[2][1], TABLE[2][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][3], TABLE[0][4], TABLE[0][5], TABLE[1][3], TABLE[1][4], TABLE[1][5], TABLE[2][3], TABLE[2][4], TABLE[2][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][6], TABLE[0][7], TABLE[0][8], TABLE[1][6], TABLE[1][7], TABLE[1][8], TABLE[2][6], TABLE[2][7], TABLE[2][8]);
    }else if selected_table == 1{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[0][0], TABLE[0][1], TABLE[0][2], TABLE[1][0], TABLE[1][1], TABLE[1][2], TABLE[2][0], TABLE[2][1], TABLE[2][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[0][3], TABLE[0][4], TABLE[0][5], TABLE[1][3], TABLE[1][4], TABLE[1][5], TABLE[2][3], TABLE[2][4], TABLE[2][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[0][6], TABLE[0][7], TABLE[0][8], TABLE[1][6], TABLE[1][7], TABLE[1][8], TABLE[2][6], TABLE[2][7], TABLE[2][8]);
    }else if selected_table == 2{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[0][0], TABLE[0][1], TABLE[0][2], TABLE[1][0], TABLE[1][1], TABLE[1][2], TABLE[2][0], TABLE[2][1], TABLE[2][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[0][3], TABLE[0][4], TABLE[0][5], TABLE[1][3], TABLE[1][4], TABLE[1][5], TABLE[2][3], TABLE[2][4], TABLE[2][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[0][6], TABLE[0][7], TABLE[0][8], TABLE[1][6], TABLE[1][7], TABLE[1][8], TABLE[2][6], TABLE[2][7], TABLE[2][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][0], TABLE[0][1], TABLE[0][2], TABLE[1][0], TABLE[1][1], TABLE[1][2], TABLE[2][0], TABLE[2][1], TABLE[2][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][3], TABLE[0][4], TABLE[0][5], TABLE[1][3], TABLE[1][4], TABLE[1][5], TABLE[2][3], TABLE[2][4], TABLE[2][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[0][6], TABLE[0][7], TABLE[0][8], TABLE[1][6], TABLE[1][7], TABLE[1][8], TABLE[2][6], TABLE[2][7], TABLE[2][8]);
    }


    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");


    if selected_table == 3{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][0], TABLE[3][1], TABLE[3][2], TABLE[4][0], TABLE[4][1], TABLE[4][2], TABLE[5][0], TABLE[5][1], TABLE[5][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][3], TABLE[3][4], TABLE[3][5], TABLE[4][3], TABLE[4][4], TABLE[4][5], TABLE[5][3], TABLE[5][4], TABLE[5][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][6], TABLE[3][7], TABLE[3][8], TABLE[4][6], TABLE[4][7], TABLE[4][8], TABLE[5][6], TABLE[5][7], TABLE[5][8]);
    }else if selected_table == 4{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[3][0], TABLE[3][1], TABLE[3][2], TABLE[4][0], TABLE[4][1], TABLE[4][2], TABLE[5][0], TABLE[5][1], TABLE[5][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[3][3], TABLE[3][4], TABLE[3][5], TABLE[4][3], TABLE[4][4], TABLE[4][5], TABLE[5][3], TABLE[5][4], TABLE[5][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[3][6], TABLE[3][7], TABLE[3][8], TABLE[4][6], TABLE[4][7], TABLE[4][8], TABLE[5][6], TABLE[5][7], TABLE[5][8]);
    }else if selected_table == 5{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[3][0], TABLE[3][1], TABLE[3][2], TABLE[4][0], TABLE[4][1], TABLE[4][2], TABLE[5][0], TABLE[5][1], TABLE[5][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[3][3], TABLE[3][4], TABLE[3][5], TABLE[4][3], TABLE[4][4], TABLE[4][5], TABLE[5][3], TABLE[5][4], TABLE[5][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[3][6], TABLE[3][7], TABLE[3][8], TABLE[4][6], TABLE[4][7], TABLE[4][8], TABLE[5][6], TABLE[5][7], TABLE[5][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][0], TABLE[3][1], TABLE[3][2], TABLE[4][0], TABLE[4][1], TABLE[4][2], TABLE[5][0], TABLE[5][1], TABLE[5][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][3], TABLE[3][4], TABLE[3][5], TABLE[4][3], TABLE[4][4], TABLE[4][5], TABLE[5][3], TABLE[5][4], TABLE[5][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[3][6], TABLE[3][7], TABLE[3][8], TABLE[4][6], TABLE[4][7], TABLE[4][8], TABLE[5][6], TABLE[5][7], TABLE[5][8]);
    }


    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");


    if selected_table == 6{
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][0], TABLE[6][1], TABLE[6][2], TABLE[7][0], TABLE[7][1], TABLE[7][2], TABLE[8][0], TABLE[8][1], TABLE[8][2]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][3], TABLE[6][4], TABLE[6][5], TABLE[7][3], TABLE[7][4], TABLE[7][5], TABLE[8][3], TABLE[8][4], TABLE[8][5]);
        println!("│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][6], TABLE[6][7], TABLE[6][8], TABLE[7][6], TABLE[7][7], TABLE[7][8], TABLE[8][6], TABLE[8][7], TABLE[8][8]);
    }else if selected_table == 7{
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[6][0], TABLE[6][1], TABLE[6][2], TABLE[7][0], TABLE[7][1], TABLE[7][2], TABLE[8][0], TABLE[8][1], TABLE[8][2]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[6][3], TABLE[6][4], TABLE[6][5], TABLE[7][3], TABLE[7][4], TABLE[7][5], TABLE[8][3], TABLE[8][4], TABLE[8][5]);
        println!("│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │{}╎{}╎{}│ │", TABLE[6][6], TABLE[6][7], TABLE[6][8], TABLE[7][6], TABLE[7][7], TABLE[7][8], TABLE[8][6], TABLE[8][7], TABLE[8][8]);
    }else if selected_table == 8{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[6][0], TABLE[6][1], TABLE[6][2], TABLE[7][0], TABLE[7][1], TABLE[7][2], TABLE[8][0], TABLE[8][1], TABLE[8][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[6][3], TABLE[6][4], TABLE[6][5], TABLE[7][3], TABLE[7][4], TABLE[7][5], TABLE[8][3], TABLE[8][4], TABLE[8][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │\x1b[93m{}|{}|{}\x1b[0m│ │", TABLE[6][6], TABLE[6][7], TABLE[6][8], TABLE[7][6], TABLE[7][7], TABLE[7][8], TABLE[8][6], TABLE[8][7], TABLE[8][8]);
    }else{
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][0], TABLE[6][1], TABLE[6][2], TABLE[7][0], TABLE[7][1], TABLE[7][2], TABLE[8][0], TABLE[8][1], TABLE[8][2]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][3], TABLE[6][4], TABLE[6][5], TABLE[7][3], TABLE[7][4], TABLE[7][5], TABLE[8][3], TABLE[8][4], TABLE[8][5]);
        println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │", TABLE[6][6], TABLE[6][7], TABLE[6][8], TABLE[7][6], TABLE[7][7], TABLE[7][8], TABLE[8][6], TABLE[8][7], TABLE[8][8]);
    }


    println!("└─┴─────┴─┴─────┴─┴─────┴─┘");
}

fn play(round:i8, mut selected_table:String) {
    if round==0{
        println!("Which box to start at?");
        io::stdin()
            .read_line(&mut selected_table)
            .expect("Failed to read line");
        let selected_table: i8 = selected_table.trim().parse().expect("Please type a number!");
        println!("{}", selected_table);
    }
}

fn select_table(selected_table: String){
    
}

