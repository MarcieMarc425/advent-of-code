use std::fmt::Error;

use crate::mods::parse_engine_schematics;

pub fn process(input: &str) -> Result<u32, Error> {
    let engine_schematics = parse_engine_schematics(input).unwrap();
    let part_numbers = engine_schematics.get_all_numbers_adjacent_to_symbols();
    let sum = engine_schematics
        .get_gears_ratios(part_numbers)
        .iter()
        .sum::<u32>();
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
