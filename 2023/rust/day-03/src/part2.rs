use std::fmt::Error;

use crate::mods::parse_engine_schematics;

pub fn process(input: &str) -> Result<u32, Error> {
    let engine_schematics = parse_engine_schematics(input).unwrap();
    let part_numbers = engine_schematics.get_all_numbers_adjacent_to_symbols();
    let gear_numbers_candidates = engine_schematics.get_gears(part_numbers);
    let gears = gear_numbers_candidates
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| {
            let ratio = numbers[0] * numbers[1];
            ratio
        });
    let sum = gears.sum::<u32>();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = process(input).unwrap();
        assert_eq!(output, 467835);
    }
}
