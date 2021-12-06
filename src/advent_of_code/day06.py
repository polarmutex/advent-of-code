from __future__ import annotations

import argparse
import os.path
from collections import defaultdict
from typing import Dict, List, Tuple

from support import timing

FishTimer = int
Fish = List[FishTimer]
NumFish = int

INPUT_S = """\
3,4,3,1,2
"""


def parse(input: str) -> Fish:
    fish: Fish = [int(num) for num in input.split(",")]
    return fish


def compute(input: Fish, days_to_run: int) -> int:
    total_fish: int = 0

    fish_dict: Dict[FishTimer, NumFish] = defaultdict(lambda: 0)

    # setup inital count
    for timer in input:
        fish_dict[timer] += 1

    for _ in range(days_to_run):
        next_fish_dict: Dict[FishTimer, NumFish] = defaultdict(lambda: 0)
        for timer, number in fish_dict.items():
            if timer == 0:
                next_fish_dict[8] += number
                next_fish_dict[6] += number
            else:
                next_fish_dict[timer - 1] += number
        fish_dict = next_fish_dict

    for num_fish in fish_dict.values():
        total_fish += num_fish

    return total_fish


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_S)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        fish = parse(f.read())

    print(compute(fish, 256))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
