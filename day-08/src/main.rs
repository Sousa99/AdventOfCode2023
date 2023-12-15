// Load Local Modules
mod read;

// Imports
use day_08::parse_map;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let map = parse_map(input);
    
    // Part 1
    let computed_steps = map.follow_instructions(('A', 'A', 'A'), ('Z', 'Z', 'Z'));
    println!("\r🏜️  Instructed path steps: '{}' (Part 1)", computed_steps);

    // Part 2
    let computed_ghostly_steps = map.follow_ghostly_instructions('A', 'Z');
    println!("\r🏜️  Instructed ghostly path steps: '{}' (Part 2)", computed_ghostly_steps);
}