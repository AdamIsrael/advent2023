// advent of code 2003: day 2
// https://adventofcode.com/2023/day/2

use io::BufReader;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Cube {
    colour: String,
    count: i32,
}
impl Cube {
    pub fn parse(input: &str) -> Option<Cube> {
        let mut cube = Cube {
            colour: "".to_string(),
            count: 0,
        };

        if let Some(idx) = input.find(' ') {
            let count_str = &input[..idx];
            cube.count = count_str.parse::<i32>().unwrap();
            cube.colour = input[idx + 1..].trim().to_string();
        }
        Some(cube)
    }
}

#[derive(Debug)]
struct Cubes {
    cubes: Vec<Cube>,
}
impl Cubes {
    pub fn parse(input: &str) -> Option<Cubes> {
        let mut cubes = Cubes { cubes: vec![] };
        for cube_str in input.split(',') {
            let cube = Cube::parse(cube_str.trim());
            if cube.is_some() {
                cubes.cubes.push(cube.unwrap());
            }
        }
        Some(cubes)
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Cubes>,
}
impl Game {
    /// Parse a game
    pub fn parse(input: String) -> Option<Game> {
        let mut game = Game {
            id: 0,
            sets: vec![],
        };

        if let Some(idx) = input.find(':') {
            // Get the "game #" portion of the line
            let game_str = &input[..idx];
            // Get the game ID
            let id = &game_str[game_str.rfind(' ').unwrap() + 1..];
            game.id = id.parse::<i32>().unwrap();

            // Parse the sets: iterate and parse each one
            for set in input[idx + 1..].split(';') {
                game.sets.push(Cubes::parse(set.trim()).unwrap());
            }
        }

        Some(game)
    }
}

/// For each game, find the minimum set of cubes that must have been present.
/// What is the sum of the power of these sets?
fn power_minimum_cubes(input: &mut impl io::BufRead) -> i32 {
    let mut output = 0;
    for line in input.lines() {
        let input = line.unwrap();
        let game = Game::parse(input).unwrap();
        let mut red = 1;
        let mut green = 1;
        let mut blue = 1;

        for cubes in game.sets {
            for cube in cubes.cubes {
                if cube.colour == "red" && cube.count > red {
                    red = cube.count;
                }
                if cube.colour == "green" && cube.count > green {
                    green = cube.count;
                }
                if cube.colour == "blue" && cube.count > blue {
                    blue = cube.count;
                }
            }
        }
        output += red * green * blue;
    }
    output
}

/// Return the sum of Game IDs where the sets are valid
fn sum_valid_games(input: &mut impl io::BufRead) -> i32 {
    let mut output = 0;
    let red = 12;
    let green = 13;
    let blue = 14;

    for line in input.lines() {
        let input = line.unwrap();
        let game = Game::parse(input).unwrap();
        let mut valid = true;
        for cubes in game.sets {
            for cube in cubes.cubes {
                // if the cube and colours match...
                if cube.colour == "red" && cube.count > red {
                    valid = false;
                }
                if cube.colour == "green" && cube.count > green {
                    valid = false;
                }
                if cube.colour == "blue" && cube.count > blue {
                    valid = false;
                }
            }
        }
        if valid {
            output += game.id;
        }
    }
    output
}

fn main() {
    // To iterate through stdin twice (one for each puzzle part), we'll read
    // into a vector and then create a buffered reader. I should probably benchmark
    // this vs. just passing the vec to the function.
    let mut lines: Vec<String> = vec![];
    let stdin = io::stdin();
    let lock = stdin.lock();
    for line in lock.lines() {
        let input = line.unwrap();
        lines.push(input);
    }

    let s = lines.join("\n");
    let input = s.as_bytes();
    let mut reader = BufReader::new(input);

    let output = sum_valid_games(&mut reader);
    println!("Output: {}", output);
    // first stage is 2486

    // Reset the reader
    reader = BufReader::new(input);
    let pow = power_minimum_cubes(&mut reader);
    println!("Minimum power: {}", pow);
    // second stage is 87984

}

#[cfg(test)]
mod tests {
    use super::*;
    use io::BufReader;

    #[test]
    fn day2_sum() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let s = input.join("\n");
        let input = s.as_bytes();
        let mut reader = BufReader::new(input);
        let output = sum_valid_games(&mut reader);
        assert!(output == 8);
    }

    #[test]
    fn day2_power() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let s = input.join("\n");
        let input = s.as_bytes();
        let mut reader = BufReader::new(input);

        let output = power_minimum_cubes(&mut reader);
        assert!(output == 2286);
    }
}
