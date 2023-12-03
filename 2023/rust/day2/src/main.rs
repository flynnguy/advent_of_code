use aocf::Aoc;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq)]
// struct Balls(u32, u32, u32); // red, green, blue
struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBallsError;

impl FromStr for Balls {
    type Err = ParseBallsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut subset = Balls::default();

        for color_count in s.split(", ") {
            let (count, color) = color_count.split_once(' ').expect("bad color count");
            let count = count.parse().expect("Couldn't parse");
            match color {
                "red" => subset.red = count,
                "green" => subset.green = count,
                "blue" => subset.blue = count,
                _ => (),
            }
        }
        Ok(subset)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    balls: Vec<Balls>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id_str, balls) = s.split_once(": ").expect("bad game");

        let id = game_id_str
            .split_once(' ')
            .expect("bad game prefix")
            .1
            .parse::<u32>()
            .unwrap();
        let balls = balls
            .split("; ")
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        Ok(Game { id, balls })
    }
}

// Should probably move this into a library
fn get_input() -> String {
    let mut aoc = Aoc::new()
        .year(Some(2023))
        .day(Some(2))
        .cookie_file("../../.adventofcode.session")
        .init()
        .unwrap();

    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "You probably need to set your cookie".to_string()
    }
}

fn part1(input: String, red: u32, green: u32, blue: u32) -> u32 {
    let mut total = 0;
    // let mut valid_game_ids = Vec::new();
    for line in input.split("\n") {
        if !line.is_empty() {
            let mut bad_ids = HashSet::new();
            let mut good_ids = HashSet::new();

            let game = line.parse::<Game>().unwrap();
            for ball_set in game.balls {
                if (ball_set.red <= red) && (ball_set.green <= green) && (ball_set.blue <= blue) {
                    good_ids.insert(game.id);
                } else {
                    bad_ids.insert(game.id);
                }
            }
            for id in bad_ids {
                good_ids.remove(&id);
            }
            for id in good_ids {
                total += id;
            }
        }
    }
    total
}

fn part2(input: String) -> u32 {
    let mut total = 0;
    for line in input.split("\n") {
        if !line.is_empty() {
            let mut max_balls = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
            let game = line.parse::<Game>().unwrap();
            for ball_set in &game.balls {
                if ball_set.red > max_balls["red"] {
                    max_balls.insert("red", ball_set.red);
                }
                if ball_set.blue > max_balls["blue"] {
                    max_balls.insert("blue", ball_set.blue);
                }
                if ball_set.green > max_balls["green"] {
                    max_balls.insert("green", ball_set.green);
                }
            }
            total += max_balls["red"] * max_balls["blue"] * max_balls["green"];
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    println!("Part 1 solution: {}", part1(get_input(), 12, 13, 14));
    println!("Part 2 solution: {}", part2(get_input()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input.to_string(), 12, 13, 14);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input.to_string());
        assert_eq!(result, 2286);
    }
}
