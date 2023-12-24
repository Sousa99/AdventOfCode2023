use std::collections::HashMap;


// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type CoordinateUnit = i64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coordinate {
  x: CoordinateUnit,
  y: CoordinateUnit
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PatternType {
  Ash,
  Rock,
}

pub struct Problem {
  map: HashMap<Coordinate, PatternType>,
  size_x: CoordinateUnit,
  size_y: CoordinateUnit,
}

enum SolutionDirection {
  Vertical,
  Horizontal
}

struct Solution {
  lower_bound: CoordinateUnit,
  direction: SolutionDirection
}

// ==================================================== CONSTANTS ====================================================

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

impl From<char> for PatternType {
  fn from(value: char) -> Self {
    match value {
      '.' => PatternType::Ash,
      '#' => PatternType::Rock,
      unknown_char => panic!("🚨 Char '{}' was not recognized as a valid pattern", unknown_char)
    }
  }
}

impl Problem {
  pub fn new(lines: Vec<Vec<char>>) -> Problem {
    let mut map: HashMap<Coordinate, PatternType> = HashMap::new();
    let size_x = lines.first().map_or(0, Vec::len) as CoordinateUnit;
    let size_y = lines.len() as CoordinateUnit;

    for (line_index, map_line) in lines.into_iter().enumerate() {
      for (row_index, map_element) in map_line.into_iter().enumerate() {
        let coordinate = Coordinate::new(row_index as CoordinateUnit, line_index as CoordinateUnit);
        map.insert(coordinate, PatternType::from(map_element));
      }
    }

    Problem { map, size_x: size_x, size_y }
  }

  fn check_vertical_mirror(&self, lower_bound: CoordinateUnit, difference_match: usize) -> Option<Solution> {
    let upper_bound = lower_bound + 1;
    let delta_check = CoordinateUnit::min(lower_bound - 0, self.size_x - 1 - upper_bound);

    let result: usize = (0..self.size_y).map(|test_y| {
      (0..=delta_check).map(|delta_x| {
        let first_coordinate = Coordinate::new(lower_bound - delta_x, test_y);
        let first = self.map.get(&first_coordinate).unwrap().to_owned();

        let second_coordinate = Coordinate::new(upper_bound + delta_x, test_y);
        let second = self.map.get(&second_coordinate).unwrap().to_owned();

        match first.eq(&second) {
          true => 0,
          false => 1
        }
      }).sum::<usize>()
    }).sum();

    match result == difference_match {
      true => Some(Solution { lower_bound, direction: SolutionDirection::Vertical }),
      false => None,
    }
  }

  fn check_horizontal_mirror(&self, lower_bound: CoordinateUnit, difference_match: usize) -> Option<Solution> {
    let upper_bound = lower_bound + 1;
    let delta_check = CoordinateUnit::min(lower_bound - 0, self.size_y - 1 - upper_bound);

    let result = (0..self.size_x).map(|test_x| {
      (0..=delta_check).map(|delta_y| {
        let first_coordinate = Coordinate::new(test_x, lower_bound - delta_y);
        let first = self.map.get(&first_coordinate).unwrap().to_owned();

        let second_coordinate = Coordinate::new(test_x, upper_bound + delta_y);
        let second = self.map.get(&second_coordinate).unwrap().to_owned();

        match first.eq(&second) {
          true => 0,
          false => 1
        }
      }).sum::<usize>()
    }).sum::<usize>();

    match result == difference_match {
      true => Some(Solution { lower_bound, direction: SolutionDirection::Horizontal }),
      false => None,
    }
  }

  fn find_first_mirror(&self, difference_match: usize) -> Option<Solution> {

    // Find Vertical Mirror
    let vertical_solution = (0..(self.size_x - 1)).find_map(|lower_bound| self.check_vertical_mirror(lower_bound, difference_match));
    if vertical_solution.is_some() { return vertical_solution }

    // Find Horizontal Mirror
    let horizontal_solution = (0..(self.size_y - 1)).find_map(|lower_bound| self.check_horizontal_mirror(lower_bound, difference_match));
    if horizontal_solution.is_some() { return horizontal_solution }

    None
  }

  pub fn get_summary_number(&self, difference_match: Option<usize>) -> CoordinateUnit {
    match self.find_first_mirror(difference_match.unwrap_or(0)) {
      None => panic!("🚨 No mirror line was found!"),
      Some(solution) => match solution.direction {
        SolutionDirection::Vertical => solution.lower_bound + 1,
        SolutionDirection::Horizontal => (solution.lower_bound + 1) * 100
      }
    }
  }
}