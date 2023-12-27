use std::collections::{HashMap, HashSet};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
type CoordinateUnit = i64;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
  x: CoordinateUnit,
  y: CoordinateUnit
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left
}

#[derive(Clone, Copy)]
enum Mirror {
  MirrorBackwardSlash,
  MirrorForwardSlash,
  SplitterHorizontal,
  SplitterVertical
}

pub struct ReflectionMap {
  mirrors: HashMap<Coordinate, Mirror>,
  size_x: CoordinateUnit,
  size_y: CoordinateUnit,
}

// ==================================================== CONSTANTS ====================================================
const MIRROR_EMPTY: char = '.';
const MIRROR_BACKWARD_SLASH: char = '\\';
const MIRROR_FORWARD_SLASH: char = '/';
const MIRROR_SPLITTER_HORIZONTAL: char = '-';
const MIRROR_SPLITTER_VERTICAL: char = '|';

// =============================================== AUXILIARY FUNCTIONS ===============================================


// ================================================= IMPLEMENTATIONS =================================================
impl Coordinate {
  pub fn new(x: CoordinateUnit, y: CoordinateUnit) -> Coordinate {
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

impl Direction {
  fn get_delta(self) -> Coordinate {
    match self {
      Self::Up => Coordinate::new(0, -1),
      Self::Down => Coordinate::new(0, 1),
      Self::Right => Coordinate::new(1, 0),
      Self::Left => Coordinate::new(-1, 0),
    }
  }

  fn face_mirror(self, mirror: Mirror) -> Vec<Self> {
    match (self, mirror) {
      // Direction: Up
      (Self::Up, Mirror::MirrorBackwardSlash) => vec![Self::Left],
      (Self::Up, Mirror::MirrorForwardSlash) => vec![Self::Right],
      (Self::Up, Mirror::SplitterHorizontal) => vec![Self::Left, Self::Right],
      (Self::Up, Mirror::SplitterVertical) => vec![Self::Up],
      // Direction: Down
      (Self::Down, Mirror::MirrorBackwardSlash) => vec![Self::Right],
      (Self::Down, Mirror::MirrorForwardSlash) => vec![Self::Left],
      (Self::Down, Mirror::SplitterHorizontal) => vec![Self::Left, Self::Right],
      (Self::Down, Mirror::SplitterVertical) => vec![Self::Down],
      // Direction: Right
      (Self::Right, Mirror::MirrorBackwardSlash) => vec![Self::Down],
      (Self::Right, Mirror::MirrorForwardSlash) => vec![Self::Up],
      (Self::Right, Mirror::SplitterHorizontal) => vec![Self::Right],
      (Self::Right, Mirror::SplitterVertical) => vec![Self::Up, Self::Down],
      // Direction: Left
      (Self::Left, Mirror::MirrorBackwardSlash) => vec![Self::Up],
      (Self::Left, Mirror::MirrorForwardSlash) => vec![Self::Down],
      (Self::Left, Mirror::SplitterHorizontal) => vec![Self::Left],
      (Self::Left, Mirror::SplitterVertical) => vec![Self::Up, Self::Down],
    }
  }
}

impl Mirror {
  fn from(value: char) -> Option<Self> {
    match value {
      MIRROR_EMPTY => None,
      MIRROR_BACKWARD_SLASH => Some(Mirror::MirrorBackwardSlash),
      MIRROR_FORWARD_SLASH => Some(Mirror::MirrorForwardSlash),
      MIRROR_SPLITTER_HORIZONTAL => Some(Mirror::SplitterHorizontal),
      MIRROR_SPLITTER_VERTICAL => Some(Mirror::SplitterVertical),
      unknown_char => panic!("🚨 Char '{}' was not recognized as a valid mirror position", unknown_char)
    }
  }
}

impl ReflectionMap {
  pub fn new(lines: &Vec<Vec<char>>) -> ReflectionMap {
    let mut mirrors: HashMap<Coordinate, Mirror> = HashMap::new();

    let size_x = lines.first().map_or(0, Vec::len) as CoordinateUnit;
    let size_y = lines.len() as CoordinateUnit;

    for (line_index, reflection_map_line) in lines.into_iter().enumerate() {
      for (row_index, &reflection_map_element) in reflection_map_line.into_iter().enumerate() {
        let coordinate = Coordinate::new(row_index as CoordinateUnit, line_index as CoordinateUnit);
        let mirror = Mirror::from(reflection_map_element);
        
        match mirror {
          None => (),
          Some(mirror_type) => { mirrors.insert(coordinate, mirror_type); },
        }
      }
    }

    ReflectionMap { mirrors, size_x, size_y }
  }

  fn coordinate_inside(&self, coordinate: &Coordinate) -> bool {
    let inside_x = coordinate.x >= 0 && coordinate.x < self.size_x;
    let inside_y = coordinate.y >= 0 && coordinate.y < self.size_y;
    inside_x && inside_y
  }

  pub fn get_energized_count(&self, start_position: Coordinate, direction: Direction) -> usize {
    let mut states_visited: HashSet<(Coordinate, Direction)> = HashSet::new();
    let mut states_to_process: Vec<(Coordinate, Direction)> = vec![(start_position, direction)];

    while !states_to_process.is_empty() {
      let to_process = states_to_process.pop().unwrap();
      states_visited.insert(to_process);

      // Figure out where it goes next
      let mirror = self.mirrors.get(&to_process.0);
      let new_directions = match mirror {
        None => vec![to_process.1],
        Some(mirror_type) => to_process.1.face_mirror(mirror_type.to_owned())
      };
      
      // Propagate new directions
      let new_states: Vec<(Coordinate, Direction)> = new_directions.into_iter()
        .map(|new_direction| (new_direction.get_delta(), new_direction))
        .map(|(delta, new_direction)| (to_process.0 + delta, new_direction))
        .filter(|(new_position, _)| self.coordinate_inside(new_position))
        .filter(|new_value| !states_visited.contains(new_value) && !states_to_process.contains(new_value))
        .collect();

      new_states.into_iter().for_each(|new_value| states_to_process.push(new_value));
    }

    states_visited.into_iter()
      .map(|(position, _)| position)
      .collect::<HashSet<_>>().len()
  }

  pub fn get_most_energizable_starting_point(&self) -> (Coordinate, Direction, usize) {
    let horizontal_states = (0..self.size_x).into_iter()
      .flat_map(|position_x| vec![
        (Coordinate::new(position_x, 0), Direction::Down),
        (Coordinate::new(position_x, self.size_y - 1), Direction::Up)]);
    let vertical_states = (0..self.size_y).into_iter()
      .flat_map(|position_y| vec![
        (Coordinate::new(0, position_y), Direction::Right),
        (Coordinate::new(self.size_x - 1, position_y), Direction::Left)]); 

    horizontal_states.chain(vertical_states)
      .map(|(coordinate, direction)| (coordinate, direction, self.get_energized_count(coordinate, direction)))
      .max_by_key(|&(_, _, energyzed_count)| energyzed_count)
      .unwrap()
  }
}