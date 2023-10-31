use std::io;

const TABLE:[[char;9];9] = [
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

fn main() {
    draw_table();
    println!("{}", TABLE[1][1]);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}

fn draw_table(){
//    clearscreen::clear().expect("failed to clear screen");
    println!("┌─┬─────┬─┬─────┬─┬─────┬─┐");
    println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │");
    println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │");
    println!("│ │{}╎{}╎{}│ │{}╎{}╎{}│ │{}╎{}╎{}│ │");
    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");
    println!("│ │x╎o╎x│ │x╎o╎x│ │x╎o╎x│ │");
    println!("│ │o╎x╎o│ │o╎x╎o│ │o╎x╎o│ │");
    println!("│ │x╎o╎x│ │x╎o╎x│ │x╎o╎x│ │");
    println!("├─┼─────┼─┼─────┼─┼─────┼─┤");
    println!("│ │x╎o╎x│ │x╎o╎x│ │x╎o╎x│ │");
    println!("│ │o╎x╎o│ │o╎x╎o│ │o╎x╎o│ │");
    println!("│ │x╎o╎x│ │x╎o╎x│ │x╎o╎x│ │");
    println!("└─┴─────┴─┴─────┴─┴─────┴─┘");
}

fn player(){
    
}
