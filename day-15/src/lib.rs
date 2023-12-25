use std::collections::HashMap;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
pub type HashValue = u32;
pub type FocalLength = u32;

enum CommandOperation {
  Removal,
  Addition{ focal_length: FocalLength }
}
pub struct Command {
  label: String,
  operation: CommandOperation
}

struct Lens {
  label: String,
  focal_length: FocalLength
}

pub struct BoxLine {
  boxes: HashMap<HashValue, Vec<Lens>>
}

// ==================================================== CONSTANTS ====================================================
const OPERATION_REMOVAL: char = '-';
const OPERATION_ADDITION: char = '=';

// =============================================== AUXILIARY FUNCTIONS ===============================================
pub fn custom_hash(string: &&str) -> HashValue {
  string.chars()
    .map(|characther| characther as u8 as HashValue)
    .fold(0, |acc, characther| ((acc + characther) * 17) % 256)
}

// ================================================= IMPLEMENTATIONS =================================================
impl From<&str> for CommandOperation {
  fn from(value: &str) -> Self {
    match value {
      "" => CommandOperation::Removal,
      _ => CommandOperation::Addition { focal_length: value.parse().unwrap() },
    }
  }
}

impl From<&str> for Command {
  fn from(value: &str) -> Self {
    let split: Vec<&str> = match 1 {
      _ if value.contains(OPERATION_REMOVAL) => value.split(OPERATION_REMOVAL),
      _ if value.contains(OPERATION_ADDITION) => value.split(OPERATION_ADDITION),
      _ => panic!("🚨 Weird it looks like the command does not contain either '{}' or '{}'", OPERATION_REMOVAL, OPERATION_ADDITION)
    }.collect();

    Self {
      label: split.get(0).unwrap().to_string(),
      operation: CommandOperation::from(split.get(1).unwrap().to_owned())
    }
  }
}

impl Lens {
  fn new(label: String, focal_length: FocalLength) -> Lens {
    Lens { label, focal_length }
  }
}

impl BoxLine {
  pub fn new() -> BoxLine {
    BoxLine {
      boxes: (0..256).into_iter()
        .map(|v| (v, Vec::new()))
        .collect()
    }
  }

  fn find_label_box(&self, index: HashValue, label: &str) -> Option<usize> {
    self.boxes.get(&index).unwrap()
      .iter().enumerate()
      .find(|(_, box_lens)| box_lens.label == label)
      .map(|(index, _)| index)
  }

  fn process_command_remove(&mut self, label: String) {
    let box_index = custom_hash(&label.as_str());
    let lens_index = self.find_label_box(box_index, &label);

    lens_index.map(|lens_index| {
      let lens_box = self.boxes.get_mut(&box_index).unwrap();
      lens_box.remove(lens_index);
    });
  }

  fn process_command_addition(&mut self, label: String, focal_length: FocalLength) {
    let box_index = custom_hash(&label.as_str());
    let lens_index = self.find_label_box(box_index, &label);
    let lens = Lens::new(label, focal_length);

    
    match lens_index {
      Some(lens_index) => {
        let lens_box = self.boxes.get_mut(&box_index).unwrap();
        lens_box[lens_index] = lens;
      },

      None => {
        let lens_box = self.boxes.get_mut(&box_index).unwrap();
        lens_box.push(lens);
      }
    }
  }

  pub fn process_command(&mut self, command: Command) {
    match command.operation {
      CommandOperation::Removal => self.process_command_remove(command.label),
      CommandOperation::Addition { focal_length } => self.process_command_addition(command.label, focal_length)
    }
  }

  pub fn compute_focal_length(&self) -> FocalLength {
    self.boxes.iter()
      .flat_map(|(&box_index, lenses)|
        lenses.iter().enumerate()
          .map(move |(lens_index, lens)| (box_index as FocalLength + 1) * (lens_index as FocalLength + 1) * lens.focal_length))
      .sum()
  }
}