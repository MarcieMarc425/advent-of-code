use std::{collections::HashMap, fmt::Error};

use regex::Regex;

#[derive(Debug, Default)]
pub struct EngineSchematics {
    symbols: HashMap<(u32, u32), char>, // (x, y) -> symbol, where x is the x coordinate of the symbol, and y is the y coordinate of the symbol
    numbers: HashMap<(u32, u32, u32), u32>, // (x1, x2, y) -> number, where x1 is the x coordinate of the first digit of the number, x2 is the x coordinate of the last digit of the number, and y is the y coordinate of the number
}

impl EngineSchematics {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            numbers: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, x: u32, y: u32, symbol: char) {
        self.symbols.insert((x, y), symbol);
    }

    pub fn add_number(&mut self, x1: u32, x2: u32, y: u32, number: u32) {
        self.numbers.insert((x1, x2, y), number);
    }

    pub fn get_all_adjacent_coordinates(&self, x1: u32, x2: u32, y: u32) -> Vec<(u32, u32)> {
        let mut coordinates = Vec::new();

        for x in x1..=x2 {
            if x == x1 && x != 0 {
                if y != 0 {
                    coordinates.push((x - 1, y - 1));
                }
                coordinates.push((x - 1, y));
                coordinates.push((x - 1, y + 1));
            }
            if x == x2 {
                if y != 0 {
                    coordinates.push((x + 1, y - 1));
                }
                coordinates.push((x + 1, y));
                coordinates.push((x + 1, y + 1));
            }

            if y != 0 {
                coordinates.push((x, y - 1));
            }
            coordinates.push((x, y + 1));
        }

        coordinates
    }

    pub fn get_all_numbers_adjacent_to_symbols(&self) -> HashMap<(u32, u32, u32), u32> {
        let mut numbers = HashMap::new();

        for ((x1, x2, y), number) in self.numbers.iter() {
            let mut is_adjacent = false;

            for (x, y) in self.get_all_adjacent_coordinates(*x1, *x2, *y) {
                if self.symbols.contains_key(&(x, y)) {
                    is_adjacent = true;
                    break;
                }
            }

            if is_adjacent {
                numbers.insert((*x1, *x2, *y), *number);
            }
        }

        numbers
    }

    pub fn get_gears_ratios(&self, part_numbers: HashMap<(u32, u32, u32), u32>) -> Vec<u32> {
        let mut gear_hashmap: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

        for ((x1, x2, y), number) in part_numbers.iter() {
            for (x, y) in self.get_all_adjacent_coordinates(*x1, *x2, *y) {
                if self.symbols.contains_key(&(x, y)) && self.symbols[&(x, y)] == '*' {
                    let gear_numbers = gear_hashmap.entry((x, y)).or_insert(Vec::new());
                    gear_numbers.push(*number);

                    break;
                }
            }
        }

        let gear_ratios = gear_hashmap
            .iter()
            .filter(|(_, numbers)| numbers.len() == 2)
            .map(|(_, numbers)| {
                let ratio = numbers[0] * numbers[1];
                ratio
            })
            .collect::<Vec<u32>>();

        gear_ratios
    }
}

pub fn parse_engine_schematics(input: &str) -> Result<EngineSchematics, Error> {
    let mut engine_schematics = EngineSchematics::new();

    // regex for whole numbers
    let whole_number_regex = Regex::new(r"\d+").unwrap();
    // regex for symbols (not matching numbers or dots or new lines)
    let symbol_regex = Regex::new(r"[^\d\n.]").unwrap();

    input.lines().enumerate().for_each(|(line_index, line)| {
        let y = u32::try_from(line_index).expect("line_index is too large");
        whole_number_regex
            .find_iter(line)
            .enumerate()
            .for_each(|(_, m)| {
                let number = m.as_str().parse::<u32>().unwrap();
                let x1 = u32::try_from(m.start()).expect("m.start() is too large");
                let x2 = u32::try_from(m.end() - 1).expect("m.end() is too large");
                engine_schematics.add_number(x1, x2, y, number);
            });
        symbol_regex.find_iter(line).enumerate().for_each(|(_, m)| {
            let symbol = m.as_str().chars().next().unwrap();
            let x = u32::try_from(m.start()).expect("m.start() is too large");
            engine_schematics.add_symbol(x, y, symbol);
        });
    });
    Ok(engine_schematics)
}
