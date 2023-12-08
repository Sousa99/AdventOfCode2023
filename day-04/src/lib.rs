use std::collections::{HashSet, HashMap};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================
type CardNumber = u32;
type Number = u32;

pub struct Card {
  winning_numbers: HashSet<Number>,
  drawn_numbers: HashSet<Number>
}

pub type Deck = HashMap<CardNumber, Card>;

// =============================================== AUXILIARY FUNCTIONS ===============================================
pub fn parse_card(line: String) -> (CardNumber, Card) {
  // Split on ': ' to achieve ['Card X', '<winning-numbers> | '<drawn-numbers>']
  let mut information_splitted: Vec<&str> = line.split(": ").collect();
  let number: CardNumber = information_splitted.remove(0)
    .replace("Card", "")
    .trim()
    .parse().unwrap();

  // Split on ' | ' to achieve ['<winning-numbers>', '<drawn-numbers>']
  let mut information_splitted: Vec<&str> = information_splitted.remove(0)
    .split(" | ")
    .collect();
  let winning_numbers: HashSet<Number> = information_splitted.remove(0)
    .split_whitespace()
    .into_iter()
    .map(|number| number.parse().unwrap())
    .collect();
  let drawn_numbers: HashSet<Number> = information_splitted.remove(0)
    .split_whitespace()
    .into_iter()
    .map(|number| number.parse().unwrap())
    .collect();

  let card = Card { winning_numbers, drawn_numbers };
  ( number, card )
}

pub fn count_cards(card_deck: &Deck) -> usize {
  let mut card_numbers: Vec<CardNumber> = card_deck.iter()
    .map(|(&number, _)| number)
    .collect();
  card_numbers.sort();

  // Map to hold final count of cards
  let mut counter_map: HashMap<CardNumber, usize> = card_numbers.iter()
    .map(|&number| (number, 1))
    .collect();

  // Iterativelly update count of cards
  card_numbers.iter()
    .for_each(|&number| {
      
      // Get number of copies for current card
      let copies = counter_map.get(&number).unwrap().to_owned();
      // Get current card value
      let card = card_deck.get(&number).unwrap();
      let card_value = card.get_matching_numbers().len() as u32;

      for number_to_update in (number + 1)..=(number + card_value) {
        let update_number = counter_map.get_mut(&number_to_update).unwrap();
        *update_number = *update_number + copies as usize;
      }
    });

  // Count total number of cards
  counter_map.into_iter()
    .map(|(_, count)| count)
    .sum()
}

// ================================================= IMPLEMENTATIONS =================================================
impl Card {

  fn get_matching_numbers(&self) -> HashSet<Number> {
    self.drawn_numbers.intersection(&self.winning_numbers)
      .map(|number| number.to_owned())
      .collect()
  }

  pub fn get_card_value(&self) -> u32 {
    let count_matching_numbers: u32 = self.get_matching_numbers().len() as u32;

    match count_matching_numbers {
      0 => 0,
      number => (2 as u32).pow(number - 1)
    }
  }
}