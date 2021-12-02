from __future__ import annotations

import argparse
import os.path

import pytest

from support import timing

INPUT_TXT = os.path.join(os.path.dirname(__file__), "input.txt")


def compute(s: str) -> int:
    horizontal: int = 0
    depth: int = 0
    aim: int = 0

    lines = s.splitlines()
    for line in lines:
        command: str = ""
        num: int = 0

        temp = line.split(" ")
        command = temp[0]
        num = int(temp[1])

        if command == "forward":
            horizontal += num
            depth += aim * num
        elif command == "down":
            aim += num
        elif command == "up":
            aim -= num
        else:
            print("unknown command: " + command)
    return horizontal * depth


INPUT_S = """\
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 900),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s) == expected


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_TXT)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        print(compute(f.read()))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
