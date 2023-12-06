use std::collections::{HashMap, HashSet};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
type SchemaPointUnit = i32;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct SchemaPoint {
  x: SchemaPointUnit,
  y: SchemaPointUnit,
}

pub type SchemaNumberValue = u32;
type SchemaSymbolValue = char;

#[derive(Debug, PartialEq, Eq)]
enum SchemaElement {
  Blank,
  Number(SchemaNumberValue),
  Symbol(SchemaSymbolValue)
}

type SchemaMap = HashMap<SchemaPoint, usize>;

pub struct Schema {
  elements: Vec<SchemaElement>,
  map: SchemaMap,
  map_min: SchemaPoint,
  map_max: SchemaPoint,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================
pub fn parse_schema(schema_unparsed: Vec<Vec<char>>) -> Schema {
  let mut elements: Vec<SchemaElement> = Vec::new();
  let mut map: SchemaMap = HashMap::new();

  elements.push(SchemaElement::Blank);

  let map_min: SchemaPoint = SchemaPoint::new(0, 0);
  let map_max: SchemaPoint = SchemaPoint::new(schema_unparsed.len() as SchemaPointUnit, schema_unparsed.get(0).unwrap().len() as SchemaPointUnit);

  for (row_index, row) in schema_unparsed.into_iter().enumerate() {

    let mut current_value: Option<SchemaNumberValue> = None;
    for (column_index, value) in row.into_iter().enumerate() {

      let current_point = SchemaPoint::new(column_index as SchemaPointUnit, row_index as SchemaPointUnit);
      
      // Check if building current value and whether to keep building or stop
      if current_value.is_some() && !value.is_digit(10) {
        let schema_element: SchemaElement = SchemaElement::Number(current_value.unwrap());
        elements.push(schema_element);
        current_value = None;
      }

      match value {
        '.' => {
          // Insert into map the blank element;s position
          map.insert(current_point, 0);
        },

        digit if digit.is_digit(10) => {
          // Update the schema element
          let digit_parsed: SchemaNumberValue = digit.to_digit(10).unwrap();
          current_value = Some(current_value.map_or(digit_parsed, |value| value * 10 + digit_parsed));
          // Insert into map the new schema element's position
          map.insert(current_point, elements.len());
        },

        symbol => {
          // Create the schema element and add it to list
          let schema_element: SchemaElement = SchemaElement::Symbol(symbol);
          elements.push(schema_element);
          // Insert into map the new schema element's position
          map.insert(current_point, elements.len() - 1);
        } 
      };

    }

    if current_value.is_some() {
      let schema_element: SchemaElement = SchemaElement::Number(current_value.unwrap());
      elements.push(schema_element);
    }
  }


  Schema { elements, map, map_min, map_max }
}

// ================================================= IMPLEMENTATIONS =================================================
impl SchemaPoint {

  fn new(x: SchemaPointUnit, y: SchemaPointUnit) -> SchemaPoint {
    SchemaPoint { x, y }
  }
}

impl std::ops::Add for SchemaPoint {
  type Output = SchemaPoint;

  fn add(self, other: SchemaPoint) -> SchemaPoint {
    let new_x: SchemaPointUnit = self.x + other.x;
    let new_y: SchemaPointUnit = self.y + other.y;
    
    SchemaPoint::new(new_x, new_y)
  }
}

impl Schema {

  fn get_symbol_positions(&self) -> Vec<SchemaPoint> {
    self.map.iter()
      .filter(|&(_, &value)| matches!(self.elements.get(value).unwrap(), SchemaElement::Symbol(_)))
      .map(|(&position, _)| position)
      .collect()
  }

  fn get_gear_positions(&self) -> Vec<SchemaPoint> {
    self.map.iter()
      .filter_map(|(position, &value)| {
        let elem = self.elements.get(value).unwrap();
        match elem {
          SchemaElement::Symbol('*') => Some(*position),
          _ => None
        }

      })
      .collect()
  }

  fn get_surround_positions(&self, position: SchemaPoint) -> Vec<SchemaPoint> {
    let mut surround_positions: Vec<SchemaPoint> = Vec::new();
    for row_delta in -1..=1 {
      for column_delta in -1..=1 {
        if row_delta == 0 && column_delta == 0 { continue; }

        let delta = SchemaPoint::new(column_delta, row_delta);
        surround_positions.push(position + delta);
      }
    }

    surround_positions
  }

  pub fn find_part_numbers(&self) -> Vec<SchemaNumberValue> {
    // Get symbol positions
    let symbol_positions = self.get_symbol_positions();
    // Get all surround positions of symbol positions
    let check_positions: HashSet<SchemaPoint> = symbol_positions.into_iter()
      .map(|symbol_position| self.get_surround_positions(symbol_position))
      .flatten()
      .collect();

    // Get element indexes from check positions
    let element_indexes: HashSet<&usize> = check_positions.into_iter()
      .filter_map(|position| self.map.get(&position))
      .collect();

    element_indexes.into_iter()
      .map(|&index| self.elements.get(index).unwrap())
      .filter_map(|elem| {
        match elem {
          SchemaElement::Number(value) => Some(value.to_owned()),
          _ => None
        }
      })
      .collect()
  }

  pub fn find_gear_ratios(&self) -> Vec<SchemaNumberValue> {
    // Get gear positions
    let symbol_positions = self.get_gear_positions();
    // Get all surround positions of gear positions
    let check_positions: Vec<Vec<SchemaPoint>> = symbol_positions.into_iter()
      .map(|symbol_position| self.get_surround_positions(symbol_position))
      .collect();

    // Transform surround positions into surround index of elements
    let check_elements: Vec<HashSet<&usize>> = check_positions.into_iter()
      .map(|positions| positions.into_iter()
        .map(|position| self.map.get(&position).unwrap())
        .collect::<HashSet<&usize>>())
      .collect(); 

    // For each group of check positions check how many are really gears
    check_elements.into_iter()
      .map(|group| group.into_iter()
        .filter_map(|&index| {
          let elem = self.elements.get(index).unwrap();
          match elem {
            SchemaElement::Number(value) => Some(value.to_owned()),
            _ => None
          }
        })
        .collect::<Vec<SchemaNumberValue>>())
      .filter(|part_numbers| part_numbers.len() == 2)
      .map(|part_values| part_values.get(0).unwrap() * part_values.get(1).unwrap())
      .collect()
  }
}