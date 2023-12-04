use aocf::Aoc;
use std::collections::HashSet;

// Should probably move this into a library
fn get_input() -> String {
    let mut aoc = Aoc::new()
        .year(Some(2023))
        .day(Some(4))
        .cookie_file("../../.adventofcode.session")
        .init()
        .unwrap();

    if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "You probably need to set your cookie".to_string()
    }
}

fn day4(input: String, part: u8) -> usize {
    let mut part1_total = 0;
    let mut part2 = vec![1; input.lines().count()];
    for (i, card) in input.split("\n").enumerate() {
        if !card.is_empty() {
            let (_card, nums) = card.split_once(": ").expect("bad card line");
            let (winning_nums, user_nums) = nums.split_once(" | ").expect("badly formatted nums");
            let mut winning_set = HashSet::new();
            let mut user_set = HashSet::new();
            for num in winning_nums.split_whitespace() {
                winning_set.insert(num);
            }
            for num in user_nums.split_whitespace() {
                user_set.insert(num);
            }
            let winning_nums: HashSet<_> = user_set.intersection(&winning_set).collect();
            if !winning_nums.is_empty() {
                part1_total += 1 << winning_nums.len() - 1;
            }

            for j in 0..winning_nums.len() {
                part2[i + j + 1] += part2[i];
            }
        }
    }

    if part == 1 {
        part1_total
    } else {
        part2.iter().sum()
    }
}

fn main() -> std::io::Result<()> {
    println!("Part 1 solution: {}", day4(get_input(), 1));
    println!("Part 2 solution: {}", day4(get_input(), 2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = day4(input.to_string(), 1);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = day4(input.to_string(), 2);
        assert_eq!(result, 30);
    }
}
