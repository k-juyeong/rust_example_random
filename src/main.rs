use std::io::{self, BufRead};
use rand::Rng;

// #[derive(Debug)]
// enum Random {
//     Rock,
//     Paper,
//     Scissors,
// }

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // pc random choice
    let _pc = rand::thread_rng().gen_range(1..=3);
    // let _pc = match n {
    //     1 => Random::rock,
    //     2 => Random::paper,
    //     3 => Random::scissors,
    //     _ => panic!("Noooo"),
    // };


    handle.read_line(&mut buffer)?;

    buffer = (buffer.trim_end()).to_string();

    let _user: i32 = match buffer.as_str() {
        "rock" | "Rock" => 1,
        "paper" | "Paper" => 2,
        "scissors" | "Scissors" => 3,
        &_ => panic!("We're playing rock-paper-scissor right now"), 
    };

    // 1. always pc win
    // match buffer.as_str() {
    //     "rock" => println!("pc: paper"),
    //     "paper" => println!("pc: scissors"),
    //     "scissors" => println!("pc: rock"),
    //     _ => panic!("We're playing rock-paper-scissor right now"),
    // }

    // 2. pc is also player
    if _pc == _user {println!("draw");}
    
    match _pc {
        1 if _user == 2 => println!("user win!"),
        1 if _user == 3 => println!("pc win!"),
        2 if _user == 1 => println!("pc win!"),
        2 if _user == 3 => println!("user win!"),
        3 if _user == 1 => println!("user win!"),
        3 if _user == 2 => println!("pc win!"),
        _ => unreachable!("Wrong"),
    }


    Ok(())
}