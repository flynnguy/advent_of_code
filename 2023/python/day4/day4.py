#!/usr/bin/env python

import re
from aocd import get_data

def day4(lines, part=1):
    """
    Test examples

    >>> lines = ["Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", \
                 "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", \
                 "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", \
                 "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", \
                 "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", \
                 "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", \
                ]
    >>> day4(lines, 1)
    13
    >>> lines = ["Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", \
                 "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", \
                 "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", \
                 "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", \
                 "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", \
                 "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", \
                ]
    >>> day4(lines, 2)
    30
    """

    part1_total = 0
    part2 = [1] * len(lines)

    for i, card in enumerate(lines):
        card, nums = card.split(": ")
        card_id = card.split()[1]
        winning_nums = set(x for x in nums.split(" | ")[0].split())
        user_nums = set(x for x in nums.split(" | ")[1].split())
        user_winning_nums = user_nums.intersection(winning_nums)
        if len(user_winning_nums) > 0:
            part1_total += 1 << len(user_winning_nums)-1

        for j in range(len(user_winning_nums)):
            part2[i+j+1] += part2[i]

    if part == 1:
        return part1_total

    elif part == 2:

        return sum(part2)


if __name__ == "__main__":
    lines = get_data(day=4, year=2023).split("\n")

    print(f"Part 1 solution: {day4(lines, 1)}")
    print(f"Part 2 solution: {day4(lines, 2)}")
