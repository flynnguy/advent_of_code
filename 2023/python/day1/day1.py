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

def day1(part):
    total = 0
    lines = get_data(day=1, year=2023).split("\n")

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
    print(f"Part 1 solution: {day1(1)}") # Should be 54630
    print(f"Part 2 solution: {day1(2)}") # Should be 54770