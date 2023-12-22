// Load Local Modules
mod read;

// Imports
use day_10::PipeMap;

fn main() {

    let input = read::read_chars("input.txt".to_owned());
    
    let mut map = PipeMap::new(input);
    map.compute_solution();
    let map_loop = map.find_loop();
    
    // Part 1
    let max_distance = map.find_loop_distances(&map_loop).into_iter()
        .max_by_key(|elem| elem.1)
        .unwrap();
    println!("\r🔧 Coordinate with max distance: '{}' with '{}' distance (Part 1)", max_distance.0, max_distance.1);

    // Part 2
    let possible_spots = map.find_inside_spots(&map_loop).len();
    println!("\r🔧 Number of free spots inside loop: '{}' (Part 2)", possible_spots);
}