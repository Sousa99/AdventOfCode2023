use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type CoordinateUnit = i64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Coordinate {
  x: CoordinateUnit,
  y: CoordinateUnit
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlatformPosition {
  Empty,
  RoundRock,
  SquareRock
}

pub struct Platform {
  square_rocks: HashSet<Coordinate>,
  round_rocks: HashSet<Coordinate>,
  size_x: CoordinateUnit,
  size_y: CoordinateUnit,
}

#[derive(Clone, Copy, Debug)]
pub enum TiltDirection {
  North,
  South,
  East,
  West
}

// ==================================================== CONSTANTS ====================================================

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn sort_coordinate_based_direction(direction: TiltDirection, coordinate: &Coordinate) -> CoordinateUnit {
  match direction {
    TiltDirection::North => coordinate.y,
    TiltDirection::South => - coordinate.y,
    TiltDirection::East => - coordinate.x,
    TiltDirection::West => coordinate.x
  }
}

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

impl From<char> for PlatformPosition {
  fn from(value: char) -> Self {
    match value {
      '.' => PlatformPosition::Empty,
      'O' => PlatformPosition::RoundRock,
      '#' => PlatformPosition::SquareRock,
      unknown_char => panic!("🚨 Char '{}' was not recognized as a valid platform position", unknown_char)
    }
  }
}

impl From<PlatformPosition> for char {
  fn from(value: PlatformPosition) -> Self {
    match value {
      PlatformPosition::Empty => '.',
      PlatformPosition::RoundRock => 'O',
      PlatformPosition::SquareRock => '#'
    }
  }
}

impl Platform {
  pub fn new(lines: &Vec<Vec<char>>) -> Platform {
    let mut square_rocks: HashSet<Coordinate> = HashSet::new();
    let mut round_rocks: HashSet<Coordinate> = HashSet::new();

    let size_x = lines.first().map_or(0, Vec::len) as CoordinateUnit;
    let size_y = lines.len() as CoordinateUnit;

    for (line_index, map_line) in lines.into_iter().enumerate() {
      for (row_index, &map_element) in map_line.into_iter().enumerate() {
        let coordinate = Coordinate::new(row_index as CoordinateUnit, line_index as CoordinateUnit);
        let platform_type = PlatformPosition::from(map_element);
        
        match platform_type {
          PlatformPosition::Empty => (),
          PlatformPosition::SquareRock => { square_rocks.insert(coordinate); },
          PlatformPosition::RoundRock => { round_rocks.insert(coordinate); },
        }
      }
    }

    Platform { square_rocks, round_rocks, size_x, size_y }
  }

  fn coordinate_inside(&self, coordinate: &Coordinate) -> bool {
    let inside_x = coordinate.x >= 0 && coordinate.x < self.size_x;
    let inside_y = coordinate.y >= 0 && coordinate.y < self.size_y;
    inside_x && inside_y
  }

  fn find_position_after_tilt(&self, direction: TiltDirection, start_coordinate: &Coordinate, current_round_rocks: &HashSet<Coordinate>) -> Coordinate {
    fn map_direction_delta(direction: TiltDirection) -> Coordinate {
      match direction {
        TiltDirection::North => Coordinate::new(0, -1),
        TiltDirection::South => Coordinate::new(0, 1),
        TiltDirection::East => Coordinate::new(1, 0),
        TiltDirection::West => Coordinate::new(-1, 0),
      }
    }
  
    let delta = map_direction_delta(direction);
    let mut current_coordinate = start_coordinate.to_owned();
  
    let mut test_coordinate = current_coordinate + delta;
    while self.coordinate_inside(&test_coordinate) && !self.square_rocks.contains(&test_coordinate) && !current_round_rocks.contains(&test_coordinate) {
      current_coordinate = test_coordinate;
      test_coordinate = current_coordinate + delta;
    }
    
    current_coordinate
  }

  pub fn tilt_platform(&mut self, direction: TiltDirection) {
    // Move Round Rocksosition)
    let mut round_rocks_position: Vec<Coordinate> = self.round_rocks.iter().map(&Coordinate::to_owned).collect();
    round_rocks_position.sort_by_key(|elem: &Coordinate| sort_coordinate_based_direction(direction, elem));

    self.round_rocks = round_rocks_position.into_iter()
      .fold(HashSet::<Coordinate>::new(), |mut acc, coordinate| {
        let new_position = self.find_position_after_tilt(direction, &coordinate, &acc);
        acc.insert(new_position);
        acc
      }).into_iter()
      .collect();
  }

  pub fn compute_load(&self, direction: TiltDirection) -> CoordinateUnit {
    fn compute_position_load(position: &Coordinate, size_x: CoordinateUnit, size_y: CoordinateUnit, direction: TiltDirection) -> CoordinateUnit {
      match direction {
        TiltDirection::North => size_y - position.y,
        TiltDirection::South => size_y + 1,
        TiltDirection::East => size_x + 1,
        TiltDirection::West => size_x - position.x,
      }
    }

    self.round_rocks.iter()
      .map(|position| compute_position_load(position, self.size_x, self.size_y, direction))
      .sum()
  }

  pub fn cycle(&mut self) {
    let directions_vec = vec![TiltDirection::North, TiltDirection::West, TiltDirection::South, TiltDirection::East];
    for direction in directions_vec.into_iter() {
      self.tilt_platform(direction);
    }
  }

  pub fn n_cycles(&mut self, cycles: usize) {
    let mut cycle_count: usize = 0;
    let mut cheat_sheet: HashMap<u64, (Option<usize>, Option<usize>)> = HashMap::new();

    while cycle_count != cycles {
      // Do cycle
      self.cycle();
      cycle_count = cycle_count + 1;

      // Hash and Save
      let mut hasher = DefaultHasher::new();
      let mut round_rocks: Vec<Coordinate> = self.round_rocks.iter().map(&Coordinate::to_owned).collect();
      round_rocks.sort();
      round_rocks.iter().for_each(|&coordinate| coordinate.hash(&mut hasher));
      let hash = hasher.finish();

      let value = cheat_sheet.entry(hash).or_insert((None, None));
      match value {
        (None, None) => value.borrow_mut().0 = Some(cycle_count),
        (Some(_), None) => value.borrow_mut().1 = Some(cycle_count),
        _ => panic!("🚨 Should have stopped by now!")
      }

      if value.0.is_some() && value.1.is_some() { break }
    }

    if cycle_count == cycles { return }
    let found_pattern = cheat_sheet.iter()
      .find(|(_, p)| p.0.is_some() && p.1.is_some())
      .map(|(_, p)| (p.0.unwrap(), p.1.unwrap()))
      .unwrap();
    let constant =  found_pattern.0;
    let coefficient = found_pattern.1 - found_pattern.0;

    let closest_factor = (((cycles - constant) as f64) / coefficient as f64).floor() as usize;
    let closest = closest_factor * coefficient + constant;
    let missing = cycles - closest;
    for _ in 0..missing { self.cycle() }
  }
}

impl std::fmt::Display for Platform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for value_y in 0..self.size_y {
      for value_x in 0..self.size_x {
        let coordinate = Coordinate::new(value_x, value_y);
        let platform_type = match 1 {
          _ if self.round_rocks.contains(&coordinate) => PlatformPosition::RoundRock,
          _ if self.square_rocks.contains(&coordinate) => PlatformPosition::SquareRock,
          _ => PlatformPosition::Empty
        };

        let _ = write!(f, "{}", char::from(platform_type));
      }

      let _ = writeln!(f, "");
    }

    Ok(())
  }
}