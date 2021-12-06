from __future__ import annotations

import argparse
import os.path
from typing import List

from support import timing

INPUT_TXT = os.path.join(os.path.dirname(__file__), "aoc_data", "day01.txt")

INPUT_S = """\
199
200
208
210
200
207
240
269
260
263
"""


def parse(input: str) -> List[int]:
    return [int(line) for line in input.splitlines()]


def compute(input: List[int], sliding_window: int) -> int:
    incresed = 0
    for index in range(sliding_window, len(input)):

        window_1 = input[index - 1]
        for k in range(2, sliding_window + 1):
            window_1 += input[index - k]

        window_2 = input[index]
        for j in range(1, sliding_window):
            window_2 += input[index - j]

        if window_2 > window_1:
            incresed += 1

    return incresed


def solve(puzzle_input: str):
    """Solve the puzzle for the given input"""
    data = parse(puzzle_input)
    solution1 = compute(data, 1)
    solution2 = compute(data, 3)
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
