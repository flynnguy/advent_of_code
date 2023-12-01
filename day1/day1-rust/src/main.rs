use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_total(part: u32) -> u32 {
    let mut total: u32 = 0;
    let numbers = HashMap::from([
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
    if let Ok(lines) = read_lines("../input.txt") {
        for line in lines {
            let mut values = Vec::new();
            if let Ok(word) = line {
                for (i, x) in word.chars().enumerate() {
                    if x.is_digit(10) {
                        values.push(x);
                    } else if part == 2 {
                        for foo in numbers.keys() {
                            if word[i..].starts_with(foo) {
                                // println!("{:?}", numbers.get(foo).unwrap());
                                values.push(*numbers.get(foo).unwrap());
                            }
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
