// Load Local Modules
mod read;

// Imports
use day_09::ValueReading;
use day_09::ValueVariation;
use day_09::compute_variations_steps;
use day_09::estimate_next_value;
use day_09::estimate_prev_value;

fn main() {

    let input = read::read_list_int_lines("input.txt".to_owned(), " ");
    let variation_steps: Vec<Vec<ValueVariation>> = input.into_iter().map(compute_variations_steps).collect();
    
    // Part 1
    let sum_estimated_next_values: ValueReading = variation_steps.iter().map(estimate_next_value).sum();
    println!("\r🧮 Sum of values for next estimations: '{}' (Part 1)", sum_estimated_next_values);

    // Part 2
    let sum_estimated_prev_values: ValueReading = variation_steps.iter().map(estimate_prev_value).sum();
    println!("\r🧮 Sum of values for previous estimations: '{}' (Part 2)", sum_estimated_prev_values);
}