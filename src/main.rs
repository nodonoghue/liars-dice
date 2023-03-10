use std::io;

struct Player {
    player_number: u8,
    remaining_dice: u8,
    last_rolls: Vec<u8>,
    first_turn_last_die: bool
}

impl Player {
    fn lose_die(&mut self) {
        self.remaining_dice -= 1;
    }

    fn roll_dice(&mut self) {
        let mut i = 0;
        while i <= self.remaining_dice {
            //rand roll 1-6 for current die
            //add to last_rolls vector
            i += 1;
        }
    }
}

fn main() {
    println!("Welcome to Liar's Dice!");
    println!("=========================================");
    println!("How many players?");
    let num_players: i32 = input_player_count();
    println!("There will be {num_players} players.");
}

fn input_player_count() -> i32 {
    loop {
        let num_players: String = get_input();

        //check that there is an input, check that the input is numeric
        if check_input(num_players.clone()) {
            break num_players.trim().parse().expect("Error");
        } else {
            println!("Error reading input, please enter the number of players as a number between 2 and 6.");
        }
    }
}

fn get_input() -> String {
    let mut num_players = String::new();
    io::stdin().read_line(&mut num_players).expect("Error.");
    num_players
}

fn check_input(input: String) -> bool {
    if input.trim().parse::<i32>().is_ok() {
        let num = input.trim().parse::<i32>().expect("Error.");
        (2..=6).contains(&num)
    } else {
        false
    }
}
