// Load Local Modules
mod read;

// Imports
use day_16::Coordinate;
use day_16::Direction;
use day_16::ReflectionMap;

fn main() {

    let input = read::read_chars("input.txt".to_owned());
    let reflection_map = ReflectionMap::new(&input);
    
    // Part 1
    let start_position = Coordinate::new(0, 0);
    let start_direction = Direction::Right;
    let count_positions = reflection_map.get_energized_count(start_position, start_direction);
    println!("\r⚡ Number of different positions: '{}' (Part 1)", count_positions);

    // Part 2
    let most_energized = reflection_map.get_most_energizable_starting_point();
    println!("\r⚡ Most enerizable state: '{}' ({}, {:?}) (Part 2)", most_energized.2, most_energized.0, most_energized.1);

}