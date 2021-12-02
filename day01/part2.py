from __future__ import annotations

import argparse
import os.path

import pytest

from support import timing

INPUT_TXT = os.path.join(os.path.dirname(__file__), "input.txt")


def compute(s: str) -> int:
    numbers = [int(line) for line in s.splitlines()]
    incresed = 0
    for index in range(3, len(numbers)):
        window_1 = numbers[index - 1] + numbers[index - 2] + numbers[index - 3]
        window_2 = numbers[index] + numbers[index - 1] + numbers[index - 2]
        if window_2 > window_1:
            incresed += 1

    return incresed


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


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 5),),
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
