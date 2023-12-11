use std::cmp::Ordering;
use std::collections::HashMap;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

// In Rust order can be taken directly from the order of the variants
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
  Card2,
  Card3,
  Card4,
  Card5,
  Card6,
  Card7,
  Card8,
  Card9,
  CardT,
  CardJ,
  CardQ,
  CardK,
  CardA,
}

// In Rust order can be taken directly from the order of the variants
#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfKind,
  FullHouse,
  FourOfKind,
  FiveOfKind,
}

type Hand = Vec<Card>;
type BidUnit = u32;

#[derive(PartialEq, Eq)]
pub struct GameEntry {
  hand: Hand,
  bid: BidUnit,
}

type Game = Vec<GameEntry>;

// =============================================== AUXILIARY FUNCTIONS ===============================================
fn parse_card(card_id: char) -> Card {
  match card_id {
    '2' => Card::Card2,
    '3' => Card::Card3,
    '4' => Card::Card4,
    '5' => Card::Card5,
    '6' => Card::Card6,
    '7' => Card::Card7,
    '8' => Card::Card8,
    '9' => Card::Card9,
    'T' => Card::CardT,
    'J' => Card::CardJ,
    'Q' => Card::CardQ,
    'K' => Card::CardK,
    'A' => Card::CardA,
    symbol => panic!("🚨 Symbol '{}' not recognized as a valid card", symbol)
  }
}

fn parse_game_entry(line: &String) -> GameEntry {
  let split: Vec<&str> = line.split_whitespace().collect();

  let hand = split.get(0).unwrap().to_owned()
    .chars().into_iter()
    .map(parse_card)
    .collect();
  let bid = split.get(1).unwrap().to_owned()
    .parse().unwrap();
    
  GameEntry { hand, bid }
}

pub fn parse_game_entries(lines: &Vec<String>) -> Game {
  lines.iter()
    .map(|line| parse_game_entry(line))
    .collect()
}

fn derive_hand_type(hand: &Hand, joker: bool) -> HandType {
  let mut count_map: HashMap<Card, usize> = HashMap::new();
  for card in hand {
      let counter = count_map.entry(*card).or_insert(0);
      *counter += 1;
  }

  if joker && count_map.contains_key(&Card::CardJ) {
    let count_j = count_map.remove(&Card::CardJ).unwrap();
    let max_key = count_map.iter()
      .max_by_key(|&(_, &count)| count)
      .map(|(&card, _)| card)
      .unwrap_or(Card::CardA);

    let counter = count_map.entry(max_key).or_insert(0);
    *counter += count_j;
  }


  match count_map.len() {
    1 => HandType::FiveOfKind,
    2 if count_map.iter().any(|(_, &size)| size == 4)  => HandType::FourOfKind,
    2 => HandType::FullHouse,
    3 if count_map.iter().any(|(_, &size)| size == 3) => HandType::ThreeOfKind,
    3 => HandType::TwoPair,
    4 => HandType::OnePair,
    5 => HandType::HighCard,
    _ => panic!("🚨 This will never happen unless a hand has more than 5 cards!")
  }
}

pub fn compute_winnings(game: &Game, joker: bool) -> BidUnit {
  // Sort game entries
  let mut game_entries: Vec<&GameEntry> = game.into_iter().collect();
  game_entries.sort_by(|&entry1, &entry2| GameEntry::custom_cmp(entry1, entry2, joker));
  // Get winnings
  game_entries.into_iter().enumerate()
    .map(|(order, game)| (order as BidUnit + 1) * game.bid)
    .sum()
}

// ================================================= IMPLEMENTATIONS =================================================
impl Card {

  fn custom_cmp(&self, other: &Self, joker: bool) -> Ordering {
    if joker && matches!(self, Card::CardJ) && !matches!(other, Card::CardJ) {
      return Ordering::Less;
    } else if joker && !matches!(self, Card::CardJ) && matches!(other, Card::CardJ) {
      return Ordering::Greater;
    }

    self.cmp(other)
  }
}

impl GameEntry {

  fn custom_cmp(&self, other: &Self, joker: bool) -> Ordering {
      let self_hand_type = derive_hand_type(&self.hand, joker);
      let other_hand_type = derive_hand_type(&other.hand, joker);

      let hand_comparisson = self_hand_type.cmp(&other_hand_type);
      match hand_comparisson {
          Ordering::Equal => {
            let card_comparissons: Vec<Ordering> = self.hand.iter().zip(other.hand.iter())
              .map(|(self_card, other_card)| self_card.custom_cmp(other_card, joker))
              .filter(|card_comparisson| !matches!(card_comparisson, Ordering::Equal))
              .collect();

            card_comparissons.first().map_or(Ordering::Equal, |comparisson| comparisson.to_owned())

          },
          hand_comparisson => hand_comparisson
      }
  }
}
