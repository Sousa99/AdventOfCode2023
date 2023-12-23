use std::{hash::{Hash, Hasher}, collections::{hash_map::DefaultHasher, HashMap}};

use itertools::intersperse;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpringStatus {
  Operational,
  Damaged,
  Unknown,
}

pub type Record = Vec<SpringStatus>;

pub struct SpringRecord {
  record: Record,
  hint: Vec<usize>,
  solutions: Option<usize>
}

#[derive(Clone, Debug, Hash)]
struct SolutionBuilder {
  set_pattern: Vec<usize>,
  current_pattern: Option<usize>,
}

type SolutionCheatSheetKey = u64;
struct SolutionCheatSheet {
  map: HashMap<SolutionCheatSheetKey, usize>
}

// ==================================================== CONSTANTS ====================================================

// =============================================== AUXILIARY FUNCTIONS ===============================================

// ================================================= IMPLEMENTATIONS =================================================
impl From<char> for SpringStatus {
  fn from(value: char) -> Self {
    match value {
      '.' => SpringStatus::Operational,
      '#' => SpringStatus::Damaged,
      '?' => SpringStatus::Unknown,
      unknown_char => panic!("🚨 Char '{}' was not recognized as a valid spring status", unknown_char)
    }
  }
}

impl SpringRecord {
  pub fn new(line: &String, repetitions: usize) -> SpringRecord {
    let mut line_split = line.split_whitespace();

    // Compute record
    let record_fold: Record = line_split.next().unwrap()
      .chars().map(SpringStatus::from)
      .collect();
    let record: Record = intersperse((0..repetitions).map(|_| record_fold.clone()), vec![SpringStatus::Unknown]).flatten().collect();
    // Compute hint
    let hint_fold: Vec<usize> = line_split.next().unwrap()
      .split(",")
      .map(|number| number.parse().unwrap())
      .collect();
    let hint: Vec<usize> = (0..repetitions).map(|_| hint_fold.clone()).flatten().collect();

    SpringRecord { record, hint, solutions: None }
  }

  fn filter_current_solution(&self, solution: &SolutionBuilder) -> bool {
    let computed_pattern_size = solution.set_pattern.len();
    
    if computed_pattern_size > self.hint.len() ||
      (computed_pattern_size == self.hint.len() && solution.current_pattern.is_some()) { return false; }
    if computed_pattern_size == 0 && solution.current_pattern.is_none() { return true; }

    let hint_splice = &self.hint[0..computed_pattern_size];
    return solution.set_pattern.eq(hint_splice) &&
      solution.current_pattern.map_or(true, |pattern| pattern.le(self.hint.get(computed_pattern_size).unwrap()));
  }

  fn filter_final_solution(&self, solution: &SolutionBuilder) -> bool {
    solution.set_pattern.eq(&self.hint) && matches!(solution.current_pattern, None)
  }

  fn recursive_solution_finder(&self, current_index: usize, current_solution: SolutionBuilder, cheat_sheet: &mut SolutionCheatSheet) -> usize {
    if current_index == self.record.len() {
      let final_solution = current_solution.close_solution();
      match self.filter_final_solution(&final_solution) {
        true => return 1,
        false => return 0
      }
    }

    let current_status = self.record.get(current_index).unwrap().to_owned();
    let generated_solutions = current_solution.generate_new_solutions(current_status);

    match cheat_sheet.get_entry(current_index, &current_solution) {
      None => (),
      Some(solution) => return solution
    }

    let count_permutations = generated_solutions.into_iter()
      .filter(|new_solution| self.filter_current_solution(new_solution))
      .map(|new_solution| cheat_sheet.get_entry(current_index + 1, &new_solution)
        .unwrap_or(self.recursive_solution_finder(current_index + 1, new_solution, cheat_sheet)))
      .sum();

    cheat_sheet.add_entry(current_index, &current_solution, count_permutations);
    count_permutations
  }

  pub fn compute_solutions(&mut self) {
    let mut solver_cheat_sheet = SolutionCheatSheet::new();
    let solutions = self.recursive_solution_finder(0, SolutionBuilder::new(), &mut solver_cheat_sheet);
    self.solutions = Some(solutions)
  }

  pub fn get_solutions(&self) -> usize {
    match self.solutions {
      Some(_) => self.solutions.unwrap(),
      None => panic!("🚨 Solution not yet computed!")
    }
  }
}

impl SolutionBuilder {
  fn new() -> SolutionBuilder {
    SolutionBuilder {
      set_pattern: Vec::new(),
      current_pattern: None,
    }
  }

  fn close_solution(self) -> SolutionBuilder {
    let mut solution_operational = self.clone();
    match solution_operational.current_pattern {
      None => (),
      Some(pattern) => {
        solution_operational.set_pattern.push(pattern);
        solution_operational.current_pattern = None;
      }
    }

    solution_operational
  }

  fn generate_new_solutions(&self, new_status: SpringStatus) -> Vec<SolutionBuilder> {
    let mut solution_operational = self.clone();
    match solution_operational.current_pattern {
      None => (),
      Some(pattern) => {
        solution_operational.set_pattern.push(pattern);
        solution_operational.current_pattern = None;
      }
    }

    let mut solution_non_operational = self.clone();
    match solution_non_operational.current_pattern {
      None => solution_non_operational.current_pattern = Some(1),
      Some(pattern) => solution_non_operational.current_pattern = Some(pattern + 1),
    }

    match new_status {
      SpringStatus::Operational => vec![solution_operational],
      SpringStatus::Damaged => vec![solution_non_operational],
      SpringStatus::Unknown => vec![solution_operational, solution_non_operational],
    }
  }
}

impl SolutionCheatSheet {
  fn new() -> SolutionCheatSheet {
    SolutionCheatSheet {
      map: HashMap::new()
    }
  }

  fn compute_key(&self, entry_index: usize, entry_solution: &SolutionBuilder) -> SolutionCheatSheetKey {
    let mut hasher = DefaultHasher::new();
    (entry_index, entry_solution).hash(&mut hasher);
    hasher.finish()
  }

  fn add_entry(&mut self, entry_index: usize, entry_solution: &SolutionBuilder, count: usize) {
    let key = self.compute_key(entry_index, entry_solution);
    self.map.insert(key, count);
  }

  fn get_entry(&self, entry_index: usize, entry_solution: &SolutionBuilder) -> Option<usize> {
    let key = self.compute_key(entry_index, entry_solution);
    self.map.get(&key).map(|s| s.to_owned())
  }
}