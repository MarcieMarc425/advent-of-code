use std::{collections::HashMap, fmt::Error};

use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct Card {
    pub number: u32,
    pub winning_set: HashMap<u32, u32>, // number -> position
    pub own_set: HashMap<u32, u32>,     // number -> position
    pub matches: HashMap<u32, u32>,     // number -> position in own_set
    pub instances: u32,
}

impl Card {
    fn new(number: u32) -> Card {
        Card {
            number,
            winning_set: HashMap::new(),
            own_set: HashMap::new(),
            matches: HashMap::new(),
            instances: 1,
        }
    }

    fn upsert_winning_set(&mut self, number: u32, position: u32) {
        self.winning_set.insert(number, position);
    }

    fn upsert_own_set(&mut self, number: u32, position: u32) {
        self.own_set.insert(number, position);
    }

    fn update_num_of_matches(&mut self) {
        for (number, position) in self.winning_set.iter() {
            if let Some(own_position) = self.own_set.get(number) {
                self.matches.insert(*number, *own_position);
            }
        }
    }

    pub fn increment_instances(&mut self) {
        self.instances += 1;
    }

    pub fn get_points(&self) -> u32 {
        let num_of_matches = u32::try_from(self.matches.len()).expect("u32::try_from failed");
        let mut points: u32 = 0;

        if num_of_matches > 0 {
            let base: u32 = 2;
            points = base.pow(num_of_matches - 1);
        }

        points
    }
}

pub fn parse_cards(input: &str) -> Result<Vec<Card>, Error> {
    let mut cards = Vec::new();
    input.lines().for_each(|line| {
        let delimited_game_statement: Vec<&str> = line.split(": ").collect();
        let card_number_label = delimited_game_statement[0];
        // extract number from card_number_label
        let card_number: u32 = card_number_label
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let mut card = Card::new(card_number);
        let delimited_sets: Vec<&str> = delimited_game_statement[1].split(" | ").collect();
        let whole_number_regex = Regex::new(r"\d+").unwrap();

        whole_number_regex
            .find_iter(delimited_sets[0])
            .enumerate()
            .for_each(|(index, m)| {
                let number = m.as_str().parse::<u32>().unwrap();
                card.upsert_winning_set(number, index as u32);
            });

        whole_number_regex
            .find_iter(delimited_sets[1])
            .enumerate()
            .for_each(|(index, m)| {
                let number = m.as_str().parse::<u32>().unwrap();
                card.upsert_own_set(number, index as u32);
            });

        card.update_num_of_matches();
        cards.push(card);
    });

    Ok(cards)
}
