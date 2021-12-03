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
    line_total: int = len(lines[0])

    oxygen_rating = lines
    for index in range(line_total):
        ba = bitarray(len(oxygen_rating))
        for line_index, line in enumerate(oxygen_rating):
            ba[line_index] = int(line[index])

        result = []
        if ba.count(1) >= len(oxygen_rating) / 2:
            for o_line in oxygen_rating:
                if o_line[index] == "1":
                    result.append(o_line)
        else:
            for o_line in oxygen_rating:
                if o_line[index] == "0":
                    result.append(o_line)
        oxygen_rating = result

        if len(oxygen_rating) == 1:
            break
    print(oxygen_rating)

    co2_rating = lines
    for index in range(line_total):
        ba = bitarray(len(co2_rating))
        for line_index, line in enumerate(co2_rating):
            ba[line_index] = int(line[index])

        result = []
        if ba.count(1) < len(co2_rating) / 2:
            for c_line in co2_rating:
                if c_line[index] == "1":
                    result.append(c_line)
        else:
            for c_line in co2_rating:
                if c_line[index] == "0":
                    result.append(c_line)
        co2_rating = result

        if len(co2_rating) == 1:
            break
    print(co2_rating)

    return ba2int(bitarray(oxygen_rating[0])) * ba2int(bitarray(co2_rating[0]))


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
    ((INPUT_S, 230),),
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
