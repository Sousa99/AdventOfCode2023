use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
type CoordinateUnit = i16;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Coordinate {
  x: CoordinateUnit,
  y: CoordinateUnit
}

#[derive(Clone, Copy, EnumIter, Debug)]
pub enum PipeType {
  Unknown,
  NorthSouth,
  EastWest,
  SquareNorthEast,
  SquareNorthWest,
  SquareSouthEast,
  SquareSouthWest,
  EmptyGround
}

#[derive(Clone, Debug)]
struct Distance {
  distance: CoordinateUnit,
  prev: Vec<Coordinate>
}

struct PipeMapSolution {
  reference_point: Coordinate,
  distance_map: HashMap<Coordinate, Distance>,
}

pub struct PipeMap {
  start: Coordinate,
  map: HashMap<Coordinate, PipeType>,
  solution: Option<PipeMapSolution>
}

#[derive(Debug, Display, Clone, Copy)]
enum Direction {
  East,
  South,
  West,
  North
}

// =================================================== CONSTANTS  ===================================================
const COORDINATE_DELTA_NORTH: Coordinate  = Coordinate { x:  0, y: -1 };
const COORDINATE_DELTA_SOUTH: Coordinate  = Coordinate { x:  0, y:  1 };
const COORDINATE_DELTA_EAST: Coordinate   = Coordinate { x:  1, y:  0 };
const COORDINATE_DELTA_WEST: Coordinate   = Coordinate { x: -1, y:  0 };

// =============================================== AUXILIARY FUNCTIONS ===============================================

// ================================================= IMPLEMENTATIONS =================================================
impl Coordinate {
  fn new(x: CoordinateUnit, y: CoordinateUnit) -> Coordinate {
    Coordinate { x, y }
  }
}

impl std::ops::Add for Coordinate {
  type Output = Coordinate;

  fn add(self, other: Coordinate) -> Coordinate {
    let new_x: CoordinateUnit = self.x + other.x;
    let new_y: CoordinateUnit = self.y + other.y;
    
    Coordinate::new(new_x, new_y)
  }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
}

impl PipeType {
  fn valid_deltas_pipe(&self) -> Vec<Coordinate> {
    match self {
      PipeType::Unknown => vec![COORDINATE_DELTA_NORTH, COORDINATE_DELTA_SOUTH, COORDINATE_DELTA_EAST, COORDINATE_DELTA_WEST],
      PipeType::NorthSouth => vec![COORDINATE_DELTA_NORTH, COORDINATE_DELTA_SOUTH],
      PipeType::EastWest => vec![COORDINATE_DELTA_EAST, COORDINATE_DELTA_WEST],
      PipeType::SquareNorthEast => vec![COORDINATE_DELTA_NORTH, COORDINATE_DELTA_EAST],
      PipeType::SquareNorthWest => vec![COORDINATE_DELTA_NORTH, COORDINATE_DELTA_WEST],
      PipeType::SquareSouthEast => vec![COORDINATE_DELTA_SOUTH, COORDINATE_DELTA_EAST],
      PipeType::SquareSouthWest => vec![COORDINATE_DELTA_SOUTH, COORDINATE_DELTA_WEST],
      PipeType::EmptyGround => vec![],
    }
  }
}

impl From<char> for PipeType {
  fn from(value: char) -> Self {
    match value {
      'S' => PipeType::Unknown,
      '|' => PipeType::NorthSouth,
      '-' => PipeType::EastWest,
      'L' => PipeType::SquareNorthEast,
      'J' => PipeType::SquareNorthWest,
      'F' => PipeType::SquareSouthEast,
      '7' => PipeType::SquareSouthWest,
      '.' => PipeType::EmptyGround,
      unknown_char => panic!("🚨 Char '{}' was not recognized as a valid type", unknown_char)
    }
  }
}

impl PipeMap {
  pub fn new(map_unparsed: Vec<Vec<char>>) -> Self {
    let map: HashMap<Coordinate, PipeType> = map_unparsed.into_iter().enumerate()
      .flat_map(|(row_index, row_chars)|
        row_chars.into_iter().enumerate()
          .map(move |(column_index, char)|
            (Coordinate::new(column_index as CoordinateUnit, row_index as CoordinateUnit), PipeType::from(char))))
      .collect();
    let start = map.iter()
      .filter(|&(_, &pipe_type)| matches!(pipe_type, PipeType::Unknown))
      .last().unwrap().0.to_owned();

    PipeMap { start, map, solution: None }
  }

  fn get_movable_positions(&self, from_position: Coordinate) -> Vec<Coordinate> {
    let from_pipe_type = self.map.get(&from_position).unwrap();
    let move_positions = from_pipe_type.valid_deltas_pipe().into_iter()
      .map(|delta| from_position + delta);

    move_positions.into_iter()
      .filter(|&to_position| {
        let to_pipe_type = self.map.get(&to_position);
        match to_pipe_type {
          None => false,
          Some(to_pipe_type) => to_pipe_type.valid_deltas_pipe().into_iter()
            .map(|delta| to_position + delta)
            .any(|position| from_position == position)
        }})
      .collect()
  }

  fn compute_pipe_from_prev(&self, target_spot: Coordinate, prev_spots: HashSet<Coordinate>) -> PipeType {
    PipeType::iter()
      .map(|pipe_type| (
        pipe_type,
        pipe_type.valid_deltas_pipe().into_iter()
          .map(|delta| target_spot + delta)
          .collect::<HashSet<Coordinate>>()))
      .find(|(_, neighbours)| prev_spots.is_subset(&neighbours) && neighbours.is_subset(&prev_spots))
      .map(|(pipe_type, _)| pipe_type)
      .unwrap() 
  }

  pub fn compute_solution(&mut self) {
    // Initialize Distance Map
    let mut distance_map: HashMap<Coordinate, Distance> = HashMap::new();
    distance_map.insert(self.start, Distance { distance: 0, prev: vec![] });
    // Initialize Vector with positions to moved
    let mut moved_coordintates: HashSet<Coordinate> = HashSet::new();

    // Iterate until loop found
    let mut loop_found: Option<Coordinate> = None;
    while loop_found.is_none() {

      // Pick coordinate with lowest distance
      let (picked_key, picked_distance) = distance_map.iter()
        .filter(|&(coordinate, _)| !moved_coordintates.contains(coordinate))
        .min_by_key(|&(_, distance)| distance.distance)
        .map(|(key, value)| (key.to_owned(), value.to_owned())).unwrap();
      let picked_new_positions: Vec<Coordinate> = self.get_movable_positions(picked_key).into_iter()
        .filter(|position| !moved_coordintates.contains(position))
        .collect();

      // Update moved set
      moved_coordintates.insert(picked_key);

      // Skip if it cannot move anywhere
      if picked_new_positions.is_empty() {
        continue;
      }

      // Update distance map
      picked_new_positions.iter()
        .for_each(|new_position| {
          match distance_map.contains_key(new_position) {
            false => { distance_map.insert(new_position.to_owned(), Distance { distance: picked_distance.distance + 1, prev: vec![picked_key] }); },
            true => { distance_map.get_mut(new_position).unwrap().prev.push(picked_key); }
          }
        });

      // Does not take into consideration self loops
      loop_found = distance_map.iter()
        .find(|&(_, distance)| distance.prev.len() > 1)
        .map(|(coordinate, _)| coordinate.to_owned());
    }

    let prev_to_start: HashSet<Coordinate> = distance_map.iter()
      .filter(|&(_, distance)| distance.prev.contains(&self.start))
      .map(|(coordinate, _)| coordinate.to_owned())
      .collect();
    let start_pipe_type = self.compute_pipe_from_prev(self.start, prev_to_start);
    self.map.insert(self.start, start_pipe_type);

    self.solution = Some(
      PipeMapSolution {
        reference_point: loop_found.unwrap(),
        distance_map
      }
    );
  }

  pub fn find_loop(&self) -> HashSet<Coordinate> {

    let solution = self.solution.as_ref().unwrap();
    // Initialize structs to keep track
    let mut to_process: Vec<Coordinate> = vec![solution.reference_point.to_owned()];
    let mut found_loop: HashSet<Coordinate> = HashSet::new();

    // Iterate until nothing left to process
    while !to_process.is_empty() {
      let picked_position = to_process.pop().unwrap();
      let picked_distance = solution.distance_map.get(&picked_position).unwrap();
      // Update structs
      picked_distance.prev.iter().for_each(|prev| to_process.push(prev.to_owned()));
      found_loop.insert(picked_position);
    }

    found_loop
  }

  pub fn find_loop_distances(&self, found_loop: &HashSet<Coordinate>) -> HashSet<(Coordinate, CoordinateUnit)> {
    let solution = self.solution.as_ref().unwrap();
    found_loop.iter()
      .map(|&coordinate| (coordinate, solution.distance_map.get(&coordinate).unwrap().distance))
      .collect()
  }

  fn update_loop_direction(&self, current_position: Coordinate, direction: Direction) -> (Coordinate, Direction) {
    let current_pipe = self.map.get(&current_position).unwrap();
    match (current_pipe, direction) {
      (PipeType::Unknown, _) => panic!("🚨 At this point no spot should be unknown!"),
      (PipeType::EmptyGround, _) => panic!("🚨 At this point no empty spot should be encoutered!"),
      (PipeType::NorthSouth, Direction::North) => (current_position + COORDINATE_DELTA_NORTH, Direction::North),
      (PipeType::NorthSouth, Direction::South) => (current_position + COORDINATE_DELTA_SOUTH, Direction::South),
      (PipeType::NorthSouth, direction) => panic!("🚨 North/South is not comptible with direction '{}'", direction),
      (PipeType::EastWest, Direction::East) => (current_position + COORDINATE_DELTA_EAST, Direction::East),
      (PipeType::EastWest, Direction::West) => (current_position + COORDINATE_DELTA_WEST, Direction::West),
      (PipeType::EastWest, direction) => panic!("🚨 East/West is not comptible with direction '{}'", direction),
      (PipeType::SquareSouthEast, Direction::North) => (current_position + COORDINATE_DELTA_EAST, Direction::East),
      (PipeType::SquareSouthEast, Direction::West) => (current_position + COORDINATE_DELTA_SOUTH, Direction::South),
      (PipeType::SquareSouthEast, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
      (PipeType::SquareSouthWest, Direction::North) => (current_position + COORDINATE_DELTA_WEST, Direction::West),
      (PipeType::SquareSouthWest, Direction::East) => (current_position + COORDINATE_DELTA_SOUTH, Direction::South),
      (PipeType::SquareSouthWest, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
      (PipeType::SquareNorthEast, Direction::South) => (current_position + COORDINATE_DELTA_EAST, Direction::East),
      (PipeType::SquareNorthEast, Direction::West) => (current_position + COORDINATE_DELTA_NORTH, Direction::North),
      (PipeType::SquareNorthEast, direction) => panic!("🚨 North/East is not comptible with direction '{}'", direction),
      (PipeType::SquareNorthWest, Direction::South) => (current_position + COORDINATE_DELTA_WEST, Direction::West),
      (PipeType::SquareNorthWest, Direction::East) => (current_position + COORDINATE_DELTA_NORTH, Direction::North),
      (PipeType::SquareNorthWest, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
    }
  }

  fn expand_position_direction(&self, position: Coordinate, direction: Direction, loop_found: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let direction_delta = match direction {
      Direction::East => COORDINATE_DELTA_EAST,
      Direction::South => COORDINATE_DELTA_SOUTH,
      Direction::West => COORDINATE_DELTA_WEST,
      Direction::North => COORDINATE_DELTA_NORTH
    };

    let mut current_position = position.to_owned();
    let mut positions: HashSet<Coordinate> = HashSet::new();
    while self.map.contains_key(&current_position) && !loop_found.contains(&current_position) {
      positions.insert(current_position.to_owned());
      current_position = current_position + direction_delta;
    }

    positions
  }

  fn match_position_direction(&self, current_position: Coordinate, direction: Direction) -> Vec<(Coordinate, Direction)> {
    let current_pipe = self.map.get(&current_position).unwrap();
    match (current_pipe, direction) {
      (PipeType::Unknown, _) => panic!("🚨 At this point no spot should be unknown!"),
      (PipeType::EmptyGround, _) => panic!("🚨 At this point no empty spot should be encoutered!"),
      (PipeType::NorthSouth, Direction::North) => vec![(current_position + COORDINATE_DELTA_EAST, Direction::East)],
      (PipeType::NorthSouth, Direction::South) => vec![(current_position + COORDINATE_DELTA_WEST, Direction::West)],
      (PipeType::NorthSouth, direction) => panic!("🚨 North/South is not comptible with direction '{}'", direction),
      (PipeType::EastWest, Direction::East) => vec![(current_position + COORDINATE_DELTA_SOUTH, Direction::South)],
      (PipeType::EastWest, Direction::West) => vec![(current_position + COORDINATE_DELTA_NORTH, Direction::North)],
      (PipeType::EastWest, direction) => panic!("🚨 East/West is not comptible with direction '{}'", direction),
      (PipeType::SquareSouthEast, Direction::North) => vec![(current_position + Coordinate::new(1, 1), Direction::South), (current_position + Coordinate::new(1, 1), Direction::East)],
      (PipeType::SquareSouthEast, Direction::West) => vec![(current_position + COORDINATE_DELTA_NORTH, Direction::North), (current_position + COORDINATE_DELTA_WEST, Direction::West)],
      (PipeType::SquareSouthEast, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
      (PipeType::SquareSouthWest, Direction::North) => vec![(current_position + COORDINATE_DELTA_NORTH, Direction::North), (current_position + COORDINATE_DELTA_EAST, Direction::East)],
      (PipeType::SquareSouthWest, Direction::East) => vec![(current_position + Coordinate::new(-1, 1), Direction::South), (current_position + Coordinate::new(-1, 1), Direction::West)],
      (PipeType::SquareSouthWest, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
      (PipeType::SquareNorthEast, Direction::South) => vec![(current_position + COORDINATE_DELTA_SOUTH, Direction::South), (current_position + COORDINATE_DELTA_WEST, Direction::West)],
      (PipeType::SquareNorthEast, Direction::West) => vec![(current_position + Coordinate::new(1, -1), Direction::North), (current_position + Coordinate::new(1, -1), Direction::East)],
      (PipeType::SquareNorthEast, direction) => panic!("🚨 North/East is not comptible with direction '{}'", direction),
      (PipeType::SquareNorthWest, Direction::South) => vec![(current_position + Coordinate::new(-1, -1), Direction::North), (current_position + Coordinate::new(-1, -1), Direction::West)],
      (PipeType::SquareNorthWest, Direction::East) => vec![(current_position + COORDINATE_DELTA_SOUTH, Direction::South), (current_position + COORDINATE_DELTA_EAST, Direction::East)],
      (PipeType::SquareNorthWest, direction) => panic!("🚨 South/East is not comptible with direction '{}'", direction),
    }
  }

  pub fn find_inside_spots(&self, found_loop: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let start_position = found_loop.iter().min().unwrap().to_owned();

    let mut number_jumps: usize = 0;

    let mut current_position = start_position.to_owned();
    let mut current_direction = Direction::North;
    let mut empty_positions: HashSet<Coordinate> = HashSet::new();

    // Circunvent loop
    while number_jumps == 0 || start_position != current_position {
      let expand_configurations = self.match_position_direction(current_position, current_direction);
      expand_configurations.into_iter()
        .map(|(start_position, expand_direction)| self.expand_position_direction(start_position, expand_direction, found_loop))
        .for_each(|expand_positions| empty_positions.extend(expand_positions));

      let update = self.update_loop_direction(current_position, current_direction);
      current_position = update.0;
      current_direction = update.1;

      number_jumps += 1;
    }

    empty_positions
  }

}