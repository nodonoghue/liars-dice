use std::io;

fn main() {
    println!("Welcome to Liar's Dice!");
    println!("=========================================");
    println!("How many players?");
    let num_players: i32 = input_player_count();
    println!("There will be {num_players} players.");
}

fn input_player_count() -> i32 {
    let mut num_players = String::new();
    io::stdin()
        .read_line(&mut num_players)
        .expect("Please input a number of players.");
    num_players
        .trim()
        .parse()
        .expect("Number of players should be a number")
}
