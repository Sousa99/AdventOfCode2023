// Load Local Modules
mod read;

// Imports
use day_05::EntityID;
use day_05::parse_almanac;
use day_05::parse_almanac_range;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    
    // Part 1
    let almanac = parse_almanac(&input);
    let lowest_location: EntityID = almanac.convert_seeds_to_locations()
        .into_iter()
        .min().unwrap();
    println!("\r🌱 Lowest location value (basic version): '{}' (Part 1)", lowest_location);

    // Part 2
    let almanac_range = parse_almanac_range(&input);
    let lowest_location: EntityID = almanac_range.convert_seeds_to_locations()
        .into_iter()
        .map(|range_item| range_item.range_start)
        .min().unwrap();
    println!("\r🌱 Lowest location value (range version): '{}' (Part 2)", lowest_location);
}