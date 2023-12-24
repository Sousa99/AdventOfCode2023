// Load Local Modules
mod read;

// Imports
use day_13::Problem;

fn main() {

    let input = read::read_chars("input.txt".to_owned());
    let problems: Vec<Problem> = input.split(Vec::is_empty).into_iter()
        .map(|map_lines| Problem::new(map_lines.to_vec()))
        .collect();
    
    // Part 1
    let summary_reflection: i64 = problems.iter().map(|p| Problem::get_summary_number(p, None)).sum();
    println!("\r🪨  Sumamry reflection number: '{}' (Part 1)", summary_reflection);

    // Part 2
    let summary_reflection_diff_1: i64 = problems.iter().map(|p| Problem::get_summary_number(p, Some(1))).sum();
    println!("\r🪨  Sumamry reflection number: '{}' (Part 2)", summary_reflection_diff_1);
}