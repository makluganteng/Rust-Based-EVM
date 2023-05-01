use core::executor::Execution;
use std::io;

fn main() {
    println!("Welcome to the EVM!");
    println!("
        EEEEE  ZZZZZZZ  EEEEE  V     V  MMM   MMM
        E          ZZ   E       V   V   M M   M M
        EEEE      ZZ    EEEE     V V    M  M M  M
        E        ZZ     E         V     M   M   M
        EEEEE  ZZZZZZZ  EEEEE     V     M       M
      ");
    println!("    =================================================="); 
    println!("    | 1. Run the EVM                                  |");
    println!("    | 2. Exit                                         |");
    println!("    ==================================================");
    println!("    Please enter your choice: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number!");
    if choice == 1 {
        println!("Please enter the program to run: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let program: Vec<u8> = input
        .trim()
        .split(|c| c == ',' || c == ' ' || c == '[' || c == ']')
        .filter(|s| !s.is_empty())
        .map(|x| u8::from_str_radix(x.trim_start_matches("0x"), 16).expect("Please type a number!"))
        .collect();

        let mut execution = Execution::new();
        let result = execution.run(program);
        println!("The result of the program is: {:?}", result);
        println!("The height of the stack is: {:?}", execution.get_height());
    } else if choice == 2 {
        println!("Goodbye!");
    } else {
        println!("Invalid choice!");
    }
}
