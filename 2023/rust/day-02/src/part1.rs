use std::fmt::Error;

use crate::mods::{get_each_max_cube, get_game, get_game_number};

pub fn process(input: &str) -> Result<u32, Error> {
    let sum = get_sum(input, (12, 13, 14));
    sum
}

pub fn get_sum(input: &str, (cmp_red, cmp_green, cmp_blue): (u32, u32, u32)) -> Result<u32, Error> {
    let output = input
        .lines()
        .map(|line| {
            let game = get_game(line).unwrap();
            let (max_red, max_green, max_blue) = get_each_max_cube(&game).unwrap();

            if (max_red > cmp_red) || (max_green > cmp_green) || (max_blue > cmp_blue) {
                return 0;
            }

            return get_game_number(&game);
        })
        .sum::<u32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = process(input).unwrap();
        assert_eq!(output, 8);
    }
}
