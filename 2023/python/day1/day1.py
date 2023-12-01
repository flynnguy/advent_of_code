#!/usr/bin/env python

import re
from aocd import get_data

numbers = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}

def day1(lines, part):
    """
    Test examples

    >>> lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
    >>> day1(lines, 1)
    142

    >>> lines = ["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"]
    >>> day1(lines, 2)
    281
    """
    total = 0

    for line in lines:
        values = []
        for i, x in enumerate(line):
            if x.isdigit():
                values.append(x)
            elif part == 2:
                for word in numbers:
                    if line[i:].startswith(word):
                        values.append(numbers[word])
        if values:
            total += int(values[0] + values[-1])

    return total

if __name__ == "__main__":
    lines = get_data(day=1, year=2023).split("\n")

    print(f"Part 1 solution: {day1(lines, 1)}") # Should be 54630
    print(f"Part 2 solution: {day1(lines, 2)}") # Should be 54770
