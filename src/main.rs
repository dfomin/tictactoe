use std::io;

use tictactoe::GameBoard;

fn main() {
    let mut board = GameBoard::new();
    while !board.is_finished() {
        println!("{}", board);
        let mut input = String::new();
        println!("Player {}", board.player);
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(position) = input.trim().parse::<usize>() {
            if !board.make_move(position) {
                println!("Incorrect field");
            }
        } else {
            println!("Input number from 0 to 8");
        }
    }

    println!("{}", board);
    println!("{}", board.winner());
}
