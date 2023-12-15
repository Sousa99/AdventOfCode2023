use std::collections::{HashMap, HashSet};
use competitive_programming_rs::math::chinese_remainder_theorem::chinese_remainder_theorem;
use regex::Regex;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
type NodeItem = char;
type Node = (NodeItem, NodeItem, NodeItem);

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Direction {
  Left,
  Right
}

pub struct Map {
  instructions: Vec<Direction>,
  nodes: Vec<Node>,
  edges_map: HashMap<usize, HashMap<Direction, usize>>,
}

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn parse_instruction(char_direction: char) -> Direction {
  match char_direction {
    'L' => Direction::Left,
    'R' => Direction::Right,
    other => panic!("🚨 Characther '{}' not recognized as a valid direction.", other)
  }
}

fn parse_node(code: &str) -> Node {
  let mut chars: Vec<char> = code.chars().collect();
  (chars.remove(0), chars.remove(0), chars.remove(0))
}

pub fn parse_map(mut lines: Vec<String>) -> Map {
  // Parse first line with list of instructions
  let instructions: Vec<Direction> = lines.remove(0)
    .chars().into_iter()
    .map(parse_instruction)
    .collect();

  // Parse empty line
  lines.remove(0);

  let mut nodes: Vec<Node> = Vec::new();
  let mut edges_map: HashMap<usize, HashMap<Direction, usize>> = HashMap::new();
  // Iterate remaining lines and parse them using Regex
  let regex_expression = Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();
  lines.into_iter()
    .for_each(|line| {

      let regex_captures = regex_expression.captures(&line).unwrap();

      let source  = parse_node(regex_captures.get(1).unwrap().as_str());
      let destination_left = parse_node(regex_captures.get(2).unwrap().as_str());
      let destination_right  = parse_node(regex_captures.get(3).unwrap().as_str());

      if !nodes.contains(&source) { nodes.push(source) }
      if !nodes.contains(&destination_left) { nodes.push(destination_left) }
      if !nodes.contains(&destination_right) { nodes.push(destination_right) }

      let source_index = nodes.iter().position(|&elem| elem == source).unwrap();
      let destinaton_left_index = nodes.iter().position(|&elem| elem == destination_left).unwrap();
      let destination_right_index = nodes.iter().position(|&elem| elem == destination_right).unwrap();

      let mut directions_map: HashMap<Direction, usize> = HashMap::new();
      directions_map.insert(Direction::Left, destinaton_left_index);
      directions_map.insert(Direction::Right, destination_right_index);

      edges_map.insert(source_index, directions_map);
    });

  Map { instructions, nodes, edges_map }
}

// ================================================= IMPLEMENTATIONS =================================================
impl Map {

  fn make_iteration(&self, current_nodes: Vec<(usize, Node)>, iteration: usize) -> Vec<(usize, Node)> {
    
    let current_instruction_index = iteration % self.instructions.len();
    let current_instruction = self.instructions.get(current_instruction_index).unwrap();

    let mut chosen_neighbours = Vec::new();
    current_nodes.iter()
      .for_each(|&from_node| {
        let neighbours = self.edges_map.get(&from_node.0).unwrap();
        let neighbour_selected_index = neighbours.get(current_instruction).unwrap().to_owned();
        let neighbour_selected = self.nodes.get(neighbour_selected_index).unwrap().to_owned();

        chosen_neighbours.push((neighbour_selected_index, neighbour_selected));
      });

    chosen_neighbours
  }

  pub fn follow_instructions(&self, start_node: Node, end_node: Node) -> usize {

    let start_node_index = self.nodes.iter().position(|&elem| elem == start_node).unwrap();
    let end_node_index = self.nodes.iter().position(|&elem| elem == end_node).unwrap();

    let mut finished_path: bool = false;
    let mut current_nodes: Vec<(usize, Node)> = vec![(start_node_index, start_node)];

    // Initialize structure for path
    let mut steps: usize = 0;

    while !finished_path {

      let neighbours = self.make_iteration(current_nodes, steps);

      // Make iteration
      steps = steps + 1;
      current_nodes = neighbours;
      // Verify to finish cycle
      finished_path = current_nodes.first().unwrap().0 == end_node_index;
    }

    return steps;
  }

  pub fn follow_ghostly_instructions(&self, start_node_item: NodeItem, end_node_item: NodeItem) -> usize {

    fn check_node_item_id(node: Node, item_id: NodeItem) -> bool { node.2 == item_id }

    let mut finished_path: bool = false;
    let mut current_nodes: Vec<(usize, Node)> = self.nodes.iter().enumerate()
      .filter(|&(_, node)| check_node_item_id(*node, start_node_item))
      .map(|(node_index, &node)| (node_index, node))
      .collect();
    let mut track_final_states: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    // Initialize structure for path
    let mut steps: usize = 0;

    while !finished_path {
      
      let neighbours = self.make_iteration(current_nodes, steps);
      
      // Make iteration
      steps = steps + 1;
      current_nodes = neighbours;
      // Update track final states
      current_nodes.iter().enumerate()
        .filter(|&(_, &(_, node))| check_node_item_id(node, end_node_item))
        .for_each(|(ghost_index, &(node_index, _))| {
          let key = (ghost_index, node_index);
          if !track_final_states.contains_key(&key) { track_final_states.insert(key, Vec::new()); }

          let current_list = track_final_states.get_mut(&key).unwrap();
          current_list.push(steps);
        });
      
      // Verify to finish cycle
      let all_ghosts_identified = track_final_states.iter()
        .map(|(&(ghost_index, _), _)| ghost_index)
        .collect::<HashSet<_>>()
        .len() == current_nodes.len();
      let only_x_ghost_identified = track_final_states.len() == current_nodes.len();
      let all_ghost_two_iterations = track_final_states.iter()
        .all(|(_, iterations)| iterations.len() >= 2);
      finished_path =all_ghosts_identified && only_x_ghost_identified && all_ghost_two_iterations;
    }

    let equation_parameters: Vec<(i64, i64)> = track_final_states.into_iter()
      .map(|(_, mut iterations)| {
        let first_occurrence = iterations.remove(0) as i64;
        let second_occurrence = iterations.remove(0) as i64;
        (second_occurrence - first_occurrence, first_occurrence)})
      .collect();

    let constants: Vec<i64> = equation_parameters.iter().map(|&(_, constant)| constant).collect();
    let coefficient: Vec<i64> = equation_parameters.iter().map(|&(coefficient, _)| coefficient).collect();
    let solution = chinese_remainder_theorem(&constants, &coefficient);

    return solution.unwrap().1 as usize;
  }
}