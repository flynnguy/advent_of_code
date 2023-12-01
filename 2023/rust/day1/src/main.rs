use aocf::Aoc;
use std::collections::HashMap;

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
fn calc_total(input: String, part: u32) -> u32 {
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

    for line in input.split("\n") {
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
    println!("Part 1 solution: {}", calc_total(get_input(), 1));
    println!("Part 2 solution: {}", calc_total(get_input(), 2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = calc_total(input.to_string(), 1);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = calc_total(input.to_string(), 2);
        assert_eq!(result, 281);
    }
}
