// Load Local Modules
mod read;

// Imports
use day_06::parse_races;
use day_06::product_ways_of_winning_race;
use day_06::count_ways_solve_equation;
use day_06::parse_race_kerning;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    
    // Part 1
    let races = parse_races(&input);
    let ways_of_winning = product_ways_of_winning_race(&races).unwrap();
    println!("\r🚘 Ways of winning the competition: '{}' (Part 1)", ways_of_winning);

    // Part 2
    let race_kerning = parse_race_kerning(&input);
    let ways_of_winning_kerning = count_ways_solve_equation(race_kerning.find_way_to_beat_record()).unwrap() ;
    println!("\r🚘 Ways of winning the race: '{}' (Part 2)", ways_of_winning_kerning);
}