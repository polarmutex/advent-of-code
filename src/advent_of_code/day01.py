from __future__ import annotations

import argparse
import os.path


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


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_TXT)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        print(compute(f.read()))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
