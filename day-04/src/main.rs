
// Load Local Modules
mod read;

// Imports
use day_04::Deck;
use day_04::count_cards;
use day_04::parse_card;


fn main() {

    let input = read::read_lines("input.txt".to_owned());
    let cards: Deck = input.into_iter()
        .map(|line| parse_card(line))
        .collect();
    
    // Part 1
    let sum_card_values: u32 = cards.iter()
        .map(|(_, card)| card.get_card_value())
        .sum();
    println!("\r🃏 Sum of part values: '{}' (Part 1)", sum_card_values);

    // Part 2
    let number_cards: usize = count_cards(&cards);
    println!("\r🃏 Number of cards: '{}' (Part 2)", number_cards);
}