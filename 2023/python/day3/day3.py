#!/usr/bin/env python

import re
from aocd import get_data

def day3(lines, part=1):
    """
    Test examples

    >>> lines = ["467..114..", \
        "...*......", \
        "..35..633.", \
        "......#...", \
        "617*......", \
        ".....+.58.", \
        "..592.....", \
        "......755.", \
        "...$.*....", \
        ".664.598..", \
    ]
    >>> day3(lines, 1)
    4361

    >>> lines = ["467..114..", \
        "...*......", \
        "..35..633.", \
        "......#...", \
        "617*......", \
        ".....+.58.", \
        "..592.....", \
        "......755.", \
        "...$.*....", \
        ".664.598..", \
    ]
    >>> day3(lines, 2)
    467835
    """
    # First get the locations of the symbols
    # and for part2, the gears
    symbol_locations = set()
    gear_locations = set()
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if not(char == "." or char.isdigit()):
                symbol_locations.add((x, y))
                if char == "*":
                    gear_locations.add((x, y))

    number_list = []
    number_active = False
    current_number = ""
    new_num = 0
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if not(number_active) and char.isdigit():
                # We hit a new number
                current_number = char
                number_active = True
                new_x1 = x
            elif number_active and char.isdigit():
                # Number continues
                current_number += char
                if x == len(lines[0]) - 1:
                    # End of the line which also means end of number
                    new_num = int(current_number)
                    new_x2 = x
                    number_list.append((new_num, new_x1, new_x2, y))
                    number_active = False
                    current_number = ""
            elif number_active and (char == "." or not char.isdigit()):
                # We hit the end of the current number
                new_num = int(current_number)
                new_x2 = x - 1
                number_list.append((new_num, new_x1, new_x2, y))
                number_active = False
                current_number = ""

    if part == 1:
        part1_total = 0
        for number, x1, x2, y in number_list:
            border_coordinates = set()
            for y in range(y - 1, y + 2):
                for x in range(x1 - 1, x2 + 2):
                    border_coordinates.add((x, y))
            intersections_set = symbol_locations & border_coordinates
            if len(intersections_set) > 0:
                part1_total += number
        return part1_total

    elif part == 2:
        part2_total = 0
        for g_x, g_y in gear_locations:
            gear_borders = set()
            gear_neighbors = []
            for x in range(g_x - 1, g_x + 2):
                for y in range(g_y - 1, g_y + 2):
                    gear_borders.add((x, y))
            for number, x1, x2, y in number_list:
                if (x1, y) in gear_borders or (x2, y) in gear_borders:
                    gear_neighbors.append(number)
            if len(gear_neighbors) == 2:
                n1, n2 = gear_neighbors
                part2_total += n1 * n2

        return part2_total


if __name__ == "__main__":
    lines = get_data(day=3, year=2023).split("\n")

    print(f"Part 1 solution: {day3(lines, 1)}")
    print(f"Part 2 solution: {day3(lines, 2)}")
