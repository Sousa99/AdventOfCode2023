use std::collections::{HashSet, HashMap};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type CoordinateUnit = i64;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
  x: CoordinateUnit,
  y: CoordinateUnit
}

pub struct GalaxyMap {
  jump: CoordinateUnit,
  galaxies: Vec<Coordinate>,
}

// ==================================================== CONSTANTS ====================================================

// =============================================== AUXILIARY FUNCTIONS ===============================================

// ================================================= IMPLEMENTATIONS =================================================
impl Coordinate {
  fn new(x: CoordinateUnit, y: CoordinateUnit) -> Coordinate {
    Coordinate { x, y }
  }

  fn distance(&self, other: &Self) -> CoordinateUnit {
    let x_distance = (self.x - other.x).abs();
    let y_distance = (self.y - other.y).abs();

    x_distance + y_distance
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

impl GalaxyMap {

  const SPOT_EMPTY: char = '.';
  const SPOT_GALAXY: char = '#';

  pub fn new(lines: &Vec<Vec<char>>, jump: CoordinateUnit) -> GalaxyMap {
    let mut galaxies: Vec<Coordinate> = Vec::new();

    for (line_index, galaxy_line) in lines.into_iter().enumerate() {
      for (row_index, galaxy_char) in galaxy_line.into_iter().enumerate() {
        match galaxy_char.to_owned() {
          GalaxyMap::SPOT_EMPTY => (),
          GalaxyMap::SPOT_GALAXY => galaxies.push(Coordinate::new(row_index as CoordinateUnit, line_index as CoordinateUnit)),
          other_symbol => panic!("🚨 Symbol not recognized '{}' as a cosmic symbol", other_symbol)
        }
      }
    }

    GalaxyMap { galaxies, jump }
  }

  fn find_empty_ys(&self) -> HashSet<CoordinateUnit> {
    let current_ys: HashSet<CoordinateUnit> = self.galaxies.iter()
      .map(|coordinate| coordinate.y)
      .collect();
    let max_y: CoordinateUnit = current_ys.iter().max().unwrap().to_owned();

    (0..=max_y).into_iter().filter(|y| !current_ys.contains(y)).collect()
  }

  fn find_empty_xs(&self) -> HashSet<CoordinateUnit> {
    let current_xs: HashSet<CoordinateUnit> = self.galaxies.iter()
      .map(|coordinate| coordinate.x)
      .collect();
    let max_x: CoordinateUnit = current_xs.iter().max().unwrap().to_owned();

    (0..=max_x).into_iter().filter(|x| !current_xs.contains(x)).collect()
  }

  fn update_galaxy(&self, index: usize, empty_ys: &HashSet<CoordinateUnit>, empty_xs: &HashSet<CoordinateUnit>) -> Coordinate {
    let coordinate = self.galaxies.get(index).unwrap().to_owned();
    let delta_ys = empty_ys.iter().filter(|&&y| y < coordinate.y).count() as CoordinateUnit;
    let delta_xs = empty_xs.iter().filter(|&&x| x < coordinate.x).count() as CoordinateUnit;

    coordinate + Coordinate::new(delta_xs * (self.jump - 1), delta_ys * (self.jump - 1))
  }

  pub fn do_iteration(&mut self) {
    let empty_ys = self.find_empty_ys();
    let empty_xs = self.find_empty_xs();

    let new_galaxies: Vec<Coordinate> = (0..self.galaxies.len()).into_iter()
      .map(|galaxy_index| self.update_galaxy(galaxy_index, &empty_ys, &empty_xs))
      .collect();

    self.galaxies = new_galaxies;
  }

  pub fn compute_distances(&self) -> HashMap<(usize, usize), CoordinateUnit> {
    let number_galaxies = self.galaxies.len();
    let mut distance_map: HashMap<(usize, usize), CoordinateUnit> = HashMap::new();

    for galaxy_index in 0..number_galaxies {
      let galaxy = self.galaxies.get(galaxy_index).unwrap();
      for other_index in (galaxy_index + 1)..number_galaxies {
        let other_galaxy = self.galaxies.get(other_index).unwrap();
        let distance = galaxy.distance(other_galaxy);

        distance_map.insert((galaxy_index, other_index), distance);
      }
    }

    distance_map
  }
}