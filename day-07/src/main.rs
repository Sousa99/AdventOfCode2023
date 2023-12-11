// Load Local Modules
mod read;

// Imports
use day_07::parse_game_entries;
use day_07::compute_winnings;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let game = parse_game_entries(&input);
    
    // Part 1
    let winnings = compute_winnings(&game, false);
    println!("\r💰 Game Winnings: '{}' (Part 1)", winnings);

    // Part 2
    let winnings_joker = compute_winnings(&game, true);
    println!("\r💰 Game Winnings (with joker): '{}' (Part 2)", winnings_joker);
}