from __future__ import annotations

import argparse
import copy
import os.path
import math
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray
from enum import Enum

from support import timing

INPUT_GOOGLE = os.path.join("data", "2021", "day20_google.txt")
INPUT_GITHUB = os.path.join("data", "2021", "day20_github.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day20_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day20_twitter.txt")
INPUT_S = """\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"""


ImageEnhancementAlgo = List[int]
Image = Dict[Tuple[int, int], int]


def adjacent(x: int, y: int) -> Generator[Tuple[int, int], None, None]:
    yield x-1, y-1
    yield x, y-1
    yield x+1, y-1
    yield x-1, y
    yield x, y
    yield x+1, y
    yield x-1, y+1
    yield x, y+1
    yield x+1, y+1


def parse(input: str) -> Tuple[ImageEnhancementAlgo, Image]:
    image_algo: ImageEnhancementAlgo = []
    image: Image = collections.defaultdict(lambda: 0)

    temp = input.split("\n\n")

    for i, char in enumerate(temp[0]):
        image_algo.append(1 if char == "#" else 0)

    image_lines = temp[1].splitlines()
    for y, line in enumerate(image_lines):
        for x, char in enumerate(line):
            image[(x, y)] = 1 if char == "#" else 0

    return image_algo, image


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def print_image(image: Image) -> None:
    min_x: int = 0
    max_x: int = 0
    min_y: int = 0
    max_y: int = 0
    for key, item in image.items():
        if key[0] > max_x:
            max_x = key[0]
        elif key[0] < min_x:
            min_x = key[0]
        if key[1] > max_y:
            max_y = key[1]
        elif key[1] < min_y:
            min_y = key[1]

    print("---------------------------")
    for y in range(min_y, max_y + 1):
        line = ""
        for x in range(min_x, max_x + 1):
            line += "#" if image[(x, y)] == 1 else "."
        print(line)
    print("---------------------------")


def enhance(pt: Tuple[int, int], image: Image, algo: ImageEnhancementAlgo, background: int) -> int:
    temp = ""
    for pt in adjacent(*pt):
        if pt not in image:
            image[pt] = background
        temp += str(image[pt])
    idx = int(temp, 2)
    return algo[idx]


def compute(input: str, steps: int) -> int:
    algo, image = parse(input)
    print_image(image)
    background: int = 0
    print(f"background is {background}")

    for _ in range(steps):
        minx = min(pt[0] for pt in image)
        maxx = max(pt[0] for pt in image)
        miny = min(pt[1] for pt in image)
        maxy = max(pt[1] for pt in image)

        output_image: Image = collections.defaultdict(lambda: 0)
        for y in range(miny - 1, maxy + 2):
            for x in range(minx - 1, maxx + 2):
                output_image[(x, y)] = enhance((x, y), image, algo, background)
        if background == 0:
            background = algo[0]
            print("HERE")
        elif background == 1:
            background = algo[511]
        else:
            raise AssertionError("")
        print(f"background is {background}")
        print_image(output_image)
        image = output_image

    num_light_pixels = 0
    for value in image.values():
        if value == 1:
            num_light_pixels += 1
    return num_light_pixels


def part1(input: str) -> int:
    return compute(input, 2)


def part2(input: str) -> int:
    return compute(input, 50)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_GITHUB)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        solutions = solve(f.read())
        print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
