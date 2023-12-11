use std::str::FromStr;
use std::fmt::Debug;

use roots::Roots;
use roots::find_roots_quadratic;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type TimeUnit = u64;
pub type DistanceUnit = u64;

pub struct Race {
  allocated_time: TimeUnit,
  record_distance: DistanceUnit
}

type RaceSolution = Option<(TimeUnit, TimeUnit)>;

// =============================================== AUXILIARY FUNCTIONS ===============================================
pub fn parse_races(lines: &Vec<String>) -> Vec<Race> {

  // Parse values from each line agnostically
  fn parse_values<T>(line: &str, prefix: &str) -> Vec<T>
  where
    T: FromStr,
    <T as FromStr>::Err: Debug
  {
    line.strip_prefix(prefix).unwrap()
      .trim()
      .split_whitespace()
      .map(|time| time.parse().unwrap())
      .collect()
  }

  let times: Vec<TimeUnit> = parse_values(lines.get(0).unwrap(), "Time:");
  let distances: Vec<DistanceUnit> = parse_values(lines.get(1).unwrap(), "Distance:");

  times.into_iter().zip(distances.into_iter())
    .into_iter()
    .map(|(allocated_time, record_distance)| Race{ allocated_time, record_distance })
    .collect()
}

pub fn parse_race_kerning(lines: &Vec<String>) -> Race {

  // Parse values from each line agnostically
  fn parse_value<T>(line: &str, prefix: &str) -> T
  where
    T: FromStr,
    <T as FromStr>::Err: Debug
  {
    line.strip_prefix(prefix).unwrap()
      .trim()
      .replace(" ", "")
      .parse().unwrap()
  }

  let allocated_time: TimeUnit = parse_value(lines.get(0).unwrap(), "Time:");
  let record_distance: DistanceUnit = parse_value(lines.get(1).unwrap(), "Distance:");

  Race { allocated_time, record_distance }
}

pub fn count_ways_solve_equation(solution: RaceSolution) -> Option<u64> {
  solution.map(|solution| solution.1 - solution.0 + 1)
}

pub fn product_ways_of_winning_race(races: &Vec<Race>) -> Result<u64, &str> {
  // Get solutions to races
  let race_solutions: Vec<RaceSolution> = races.iter()
    .map(|race| race.find_way_to_beat_record())
    .collect();

  // Check if any failed at being solved
  let failed_races: Vec<usize> = race_solutions.iter().enumerate()
    .filter(|&(_, &race_solution)| race_solution.is_none())
    .map(|(race_number, _)| race_number)
    .collect();
  if failed_races.len() > 0 {
    failed_races.into_iter()
      .for_each(|race_number| println!("🚨 Race '{}' has no solution!", race_number));
    return Err("🚨 Problem could not be solved!");
  }

  // Actually compute solution 
  let race_solution = race_solutions.into_iter()
    .map(|solution| count_ways_solve_equation(solution))
    .map(|number_ways| number_ways.unwrap())
    .product();
  Ok(race_solution)
}

// ================================================= IMPLEMENTATIONS =================================================
impl Race {

  /**
   * Please refer to https://adventofcode.com/2023/day/6
   * 
   * According to the exercise, consider:
   * - t the total time given to the race
   * - d the distance to beat the record
   * - n the unknown variable which determines how long to 'charge' the car for
   * - x the distance travelled and the respective function according to n
   * 
   * So the:
   * - v0 = n
   * - x(n) = v0 * (t - n) = n * (t -n) = -n^2 + nt
   * 
   * We want to solve for:
   * - x(n) > d => -n^2 + nt - d > 0
  */ 
  pub fn find_way_to_beat_record(&self) -> RaceSolution {

    let solution = find_roots_quadratic(-1 as f64, self.allocated_time as f64, - (self.record_distance as f64));
    if let Roots::Two(solution) = solution {
      let start_range: f64 = solution[0];
      let end_range: f64 = solution[1];

      let start_range_fixed = match start_range.fract() {
        number if number != 0.0 => start_range.ceil() as TimeUnit,
        _ => start_range as TimeUnit + 1
      };

      let end_range_fixed = match end_range.fract() {
        number if number != 0.0 => end_range.floor() as TimeUnit,
        _ => end_range as TimeUnit - 1
      };

      return Some((start_range_fixed, end_range_fixed));
    }

    None
  }
}