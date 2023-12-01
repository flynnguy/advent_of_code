use aocf::Aoc;
use std::collections::HashMap;
use std::env;

// Should probably move this into a library
fn get_input() -> String {
    let mut aoc = Aoc::new()
        .year(Some(2023))
        .day(Some(1))
        .cookie_file("../../.adventofcode.session")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "You probably need to set your cookie".to_string()
    };
    input
}

// This is similar to my python solution
fn calc_total(part: u32) -> u32 {
    let mut total: u32 = 0;
    let numbers = HashMap::from([
        // values us `'` because it creates a character literal
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for line in get_input().split("\n") {
        if !line.is_empty() {
            let mut values = Vec::new();
            for (i, x) in line.chars().enumerate() {
                if x.is_digit(10) {
                    values.push(x);
                } else if part == 2 {
                    for digit in numbers.keys() {
                        if line[i..].starts_with(digit) {
                            values.push(*numbers.get(digit).unwrap());
                        }
                    }
                }
            }
            total += format!("{}{}", values[0], values[values.len() - 1])
                .parse::<u32>()
                .unwrap();
        }
    }
    total
}

fn main() -> std::io::Result<()> {
    println!("Part 1 solution: {}", calc_total(1));
    println!("Part 2 solution: {}", calc_total(2));
    Ok(())
}
