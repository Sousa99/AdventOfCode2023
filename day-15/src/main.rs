// Load Local Modules
mod read;

// Imports
use day_15::HashValue;
use day_15::custom_hash;
use day_15::Command;
use day_15::BoxLine;

fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let command_strings: Vec<&str> = input.get(0).unwrap()
        .split_terminator(",")
        .collect();
    
    // Part 1
    let hash_value: HashValue = command_strings.iter().map(custom_hash).sum();
    println!("\r💻 Hash value for initialization sequence: '{}' (Part 1)", hash_value);

    // Part 2
    let commands: Vec<Command> = command_strings.into_iter().map(Command::from).collect();
    let mut box_line: BoxLine = BoxLine::new();
    commands.into_iter().for_each(|command| box_line.process_command(command));
    let focal_length = box_line.compute_focal_length();
    println!("\r💻 Boxes' focal length: '{}' (Part 2)", focal_length);
}