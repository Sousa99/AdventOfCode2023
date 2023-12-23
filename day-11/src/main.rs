// Load Local Modules
mod read;

// Imports
use day_11::GalaxyMap;
use day_11::CoordinateUnit;

fn main() {

    let input = read::read_chars("input.txt".to_owned());
    
    // Part 1
    let mut map = GalaxyMap::new(&input, 2);
    map.do_iteration();
    let sum_shortest_paths: CoordinateUnit = map.compute_distances().into_iter()
        .map(|(_, distance)| distance).sum();
    println!("\r🌟 Sum shortest paths between galaxies: '{}' (Part 1)", sum_shortest_paths);

    // Part 2
    let jump = 1000000;
    let mut map = GalaxyMap::new(&input, jump);
    map.do_iteration();
    let sum_shortest_paths: CoordinateUnit = map.compute_distances().into_iter()
        .map(|(_, distance)| distance).sum();
    println!("\r🌟 Sum shortest paths between galaxies: '{}' with jumps of '{}' (Part 2)", sum_shortest_paths, jump);
}