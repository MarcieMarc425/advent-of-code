use std::fmt::Error;

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Cube {
    color: Color,
    number: u32,
}

#[derive(Debug)]
pub struct GameSet {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
pub struct Game {
    game_number: u32,
    game_sets: Vec<GameSet>,
}

pub fn get_game(line: &str) -> Result<Game, Error> {
    let delimited_game_statement: Vec<&str> = line.split(": ").collect();
    let game_number_label = delimited_game_statement[0];
    // extract number from game_number_label
    let game_number: u32 = game_number_label
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let game_sets: Vec<GameSet> = delimited_game_statement[1]
        .split("; ")
        .map(|game_set| {
            let cubes: Vec<Cube> = game_set
                .split(", ")
                .map(|cube| {
                    let delimited_cube: Vec<&str> = cube.split(' ').collect();
                    let color = match delimited_cube[1] {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => panic!("invalid color"),
                    };
                    let number = delimited_cube[0].parse::<u32>().expect("invalid number");
                    return Cube { color, number };
                })
                .collect();
            return GameSet { cubes };
        })
        .collect();
    let game = Game {
        game_number,
        game_sets,
    };
    Ok(game)
}

pub fn get_game_number(game: &Game) -> u32 {
    game.game_number
}

pub fn get_each_max_cube(game: &Game) -> Result<(u32, u32, u32), Error> {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for game_set in &game.game_sets {
        for cube in &game_set.cubes {
            match cube.color {
                Color::Red => {
                    if cube.number > max_red {
                        max_red = cube.number;
                    }
                }
                Color::Green => {
                    if cube.number > max_green {
                        max_green = cube.number;
                    }
                }
                Color::Blue => {
                    if cube.number > max_blue {
                        max_blue = cube.number;
                    }
                }
            }
        }
    }

    Ok((max_red, max_green, max_blue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = get_game(line).unwrap();
        assert_eq!(game.game_number, 1);
        assert_eq!(game.game_sets.len(), 3);
        assert_eq!(game.game_sets[0].cubes.len(), 2);
        assert_eq!(game.game_sets[1].cubes.len(), 3);
        assert_eq!(game.game_sets[2].cubes.len(), 1);
    }

    #[test]
    fn test_get_game_number() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = get_game(line).unwrap();
        assert_eq!(get_game_number(&game), 1);
    }

    #[test]
    fn test_get_each_max_cube() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = get_game(line).unwrap();
        let (max_red, max_green, max_blue) = get_each_max_cube(&game).unwrap();
        assert_eq!(max_red, 4);
        assert_eq!(max_green, 2);
        assert_eq!(max_blue, 6);
    }
}
