use std::fmt::Error;

use crate::mods::parse_cards;

pub fn process(input: &str) -> Result<u32, Error> {
    let cards = parse_cards(input).unwrap();
    let mut card_clone = cards.clone();
    for (index, card) in cards.iter().enumerate() {
        let num_of_matches = u32::try_from(card.matches.len()).expect("u32::try_from failed");
        let position = u32::try_from(index).expect("u32::try_from failed");
        if num_of_matches > 0 {
            for _ in 0..card_clone[index].instances {
                for m in (position + 1)..(position + num_of_matches + 1) {
                    let clone_pos = usize::try_from(m).expect("usize::try_from failed");
                    card_clone[clone_pos].increment_instances();
                }
            }
        }
    }
    let sum = card_clone.iter().map(|card| card.instances).sum::<u32>();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = process(input).unwrap();
        assert_eq!(output, 30);
    }
}
