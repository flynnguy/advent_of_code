#!/usr/bin/env python

import re
from aocd import get_data

def part1(lines, red, green, blue):
    """
    Test examples

    >>> lines = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", \
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", \
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", \
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", \
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", \
    ]
    >>> part1(lines, 12, 13, 14)
    8
    """

    game_ids = set()
    bad_game_ids = set()

    for line in lines:
        game_id = line.split(": ")[0].replace("Game ", "")
        game_sets = line.split(": ")[1].split("; ")
        for game in game_sets:
            ball_sets = game.split(", ")
            for balls in ball_sets:
                pair = balls.split(" ")
                if pair[1].lower() == 'blue' and int(pair[0]) <= blue:
                    game_ids.add(game_id)
                elif pair[1].lower() == 'red' and int(pair[0]) <= red:
                    game_ids.add(game_id)
                elif pair[1].lower() == 'green' and int(pair[0]) <= green:
                    game_ids.add(game_id)
                else:
                    bad_game_ids.add(game_id)
                    break
        game_ids -= bad_game_ids

    total = 0
    for game in game_ids:
        total += int(game)
    return total

def part2(lines):
    """
    Test examples

    >>> lines = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", \
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", \
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", \
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", \
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", \
    ]
    >>> part2(lines)
    2286
    """
    total = 0
    for line in lines:
        max_balls = {
            'red': 0,
            'blue': 0,
            'green': 0,
        }
        game_sets = line.split(": ")[1].split("; ")
        for game in game_sets:
            ball_sets = game.split(", ")
            for balls in ball_sets:
                pair = balls.split(" ")
                if int(pair[0]) > max_balls[pair[1].lower()]:
                    max_balls[pair[1].lower()] = int(pair[0])
        # This assumes there is always at least 1 type of each ball
        # Which gets me the answer so I'm not going to worry about 0 of one type
        total += max_balls['red'] * max_balls['green'] * max_balls['blue']
    return total

if __name__ == "__main__":
    lines = get_data(day=2, year=2023).split("\n")

    print(f"Part 1 solution: {part1(lines, 12, 13, 14)}") # Should be 2727
    print(f"Part 2 solution: {part2(lines)}") # Should be ??
