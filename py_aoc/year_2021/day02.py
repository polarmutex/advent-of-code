from __future__ import annotations

import argparse
import os.path
from typing import List, Tuple

from support import timing

INPUT_TXT = os.path.join("data", "2021", "day02.txt")

INPUT_S = """\
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""

CommandList = List[Tuple[str, int]]


def parse(input: str) -> CommandList:
    command_list: List[Tuple[str, int]] = []
    lines = input.splitlines()
    for line in lines:
        temp = line.split(" ")
        command: str = temp[0]
        num: int = int(temp[1])
        command_list.append((command, num))

    return command_list


def part1(input: CommandList) -> int:
    horizontal: int = 0
    depth: int = 0
    for cmd in input:

        if cmd[0] == "forward":
            horizontal += cmd[1]
        elif cmd[0] == "down":
            depth += cmd[1]
        elif cmd[0] == "up":
            depth -= cmd[1]
        else:
            print("unknown command: " + cmd[0])
    return horizontal * depth


def part2(input: CommandList) -> int:
    horizontal: int = 0
    depth: int = 0
    aim: int = 0

    for cmd in input:

        if cmd[0] == "forward":
            horizontal += cmd[1]
            depth += aim * cmd[1]
        elif cmd[0] == "down":
            aim += cmd[1]
        elif cmd[0] == "up":
            aim -= cmd[1]
        else:
            print("unknown command: " + cmd[0])
    return depth * horizontal


def solve(puzzle_input: str):
    """Solve the puzzle for the given input"""
    data = parse(puzzle_input)
    solution1 = part1(data)
    solution2 = part2(data)
    return solution1, solution2


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_TXT)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        solutions = solve(f.read())
        print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
