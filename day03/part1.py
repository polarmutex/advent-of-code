from __future__ import annotations

import argparse
import os.path
from bitarray import bitarray
from bitarray.util import ba2int
from typing import List

import pytest

from support import timing

INPUT_TXT = os.path.join(os.path.dirname(__file__), "input.txt")


def compute(s: str) -> int:
    lines = s.splitlines()
    total = len(lines)
    line_total: int = 0
    count_1: List[int] = []
    for line_index, line in enumerate(lines):
        if line_index == 0:
            line_total = len(line)

        for index, char in enumerate(line):
            if line_index == 0:
                count_1.append(0)
            if char == "1":
                count_1[index] += 1

    gamma: bitarray = bitarray(line_total)
    epsilon: bitarray = bitarray(line_total)
    for index, count in enumerate(count_1):
        if count > total / 2:
            gamma[index] = 1
            epsilon[index] = 0
        else:
            gamma[index] = 0
            epsilon[index] = 1

    return ba2int(gamma) * ba2int(epsilon)


INPUT_S = """\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 198),),
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
