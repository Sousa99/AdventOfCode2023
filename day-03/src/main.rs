
// Load Local Modules
mod read;

// Imports
use day_03::SchemaNumberValue;
use day_03::parse_schema;


fn main() {

    let input = read::read_chars("input.txt".to_owned());
    let schema = parse_schema(input);
    
    // Part 1
    let sum_part_values: SchemaNumberValue = schema.find_part_numbers()
        .into_iter()
        .sum();
    println!("\r🚇 Sum of part values: '{}' (Part 1)", sum_part_values);

    // Part 2
    let sum_gear_ratios: SchemaNumberValue = schema.find_gear_ratios()
        .into_iter()
        .sum();
    println!("\r🚇 Sum of gear ratios: '{}' (Part 2)", sum_gear_ratios);
}