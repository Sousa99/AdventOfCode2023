// Load Local Modules
mod read;

// Imports
use day_14::TiltDirection;
use day_14::Platform;

fn main() {

    let input = read::read_chars("input.txt".to_owned());
    
    // Part 1
    let mut platform: Platform = Platform::new(&input);
    platform.tilt_platform(TiltDirection::North);
    let load_north = platform.compute_load(TiltDirection::North);
    println!("\r🪨  Load on North side: '{}' (Part 1)", load_north);

    // Part 2
    let cycles: usize = 1000000000;
    let mut platform: Platform = Platform::new(&input);
    platform.n_cycles(cycles);
    let load_north = platform.compute_load(TiltDirection::North);
    println!("\r🪨  Load on North side after '{}' cycles: '{}' (Part 2)", cycles, load_north);
}