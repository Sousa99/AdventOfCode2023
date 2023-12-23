// Load Local Modules
mod read;

// Imports
use day_12::SpringRecord;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    
    // Part 1
    let mut problems: Vec<SpringRecord> = input.iter()
        .map(|line| SpringRecord::new(line, 1)).collect();
    problems.iter_mut().for_each(SpringRecord::compute_solutions);
    let sum_arrangements: usize = problems.iter().map(SpringRecord::get_solutions).sum();
    println!("\r🏥 Number of possible arrangements: '{}' (Part 1)", sum_arrangements);

    // Part 2
    let mut problems: Vec<SpringRecord> = input.iter()
        .map(|line| SpringRecord::new(line, 5)).collect();
    problems.iter_mut().for_each(SpringRecord::compute_solutions);
    let sum_arrangements: usize = problems.iter().map(SpringRecord::get_solutions).sum();
    println!("\r🏥 Number of possible arrangements: '{}' (Part 2)", sum_arrangements);
}