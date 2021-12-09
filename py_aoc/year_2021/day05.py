from __future__ import annotations

import argparse
import os.path
import re
from typing import Dict, List, Tuple

from support import timing

INPUT_TXT = os.path.join("data", "2021",  "day05.txt")

INPUT_S = """\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"""

line_re = re.compile("([0-9]+,[0-9]+)\\s?->\\s?([0-9]+,[0-9]+)")


def print_overlap(map: Dict[Tuple[int, int], int]):
    max_x: int = 0
    max_y: int = 0
    for key, item in map.items():
        if key[0] > max_x:
            max_x = key[0]
        if key[1] > max_y:
            max_y = key[1]

    print("---------------------------")
    for y in range(0, max_y + 1):
        line = ""
        for x in range(0, max_x + 1):
            if (x, y) not in map.keys():
                line += "."
            elif map[(x, y)] == 0:
                line += "."
            else:
                line += str(map[(x, y)])
        print(line)
    print("---------------------------")


Point = Tuple[int, int]
Point_Line = Tuple[Point, Point]


def parse(input: str) -> List[Point_Line]:
    lines = input.splitlines()
    line_points: List[Point_Line] = []
    for line in lines:
        matches = line_re.match(line)
        if matches is not None:
            groups = matches.groups()
            print(groups)

            point_temp = groups[0].split(",")
            point1: Point = (int(point_temp[0]), int(point_temp[1]))
            point_temp = groups[1].split(",")
            point2: Point = (int(point_temp[0]), int(point_temp[1]))
            line_points.append((point1, point2))

        else:
            assert False
    return line_points


def compute(input: List[Point_Line], process_diag: bool) -> int:
    line_cover_map: Dict[Tuple[int, int], int] = {}

    for line in input:

        point1 = line[0]
        point2 = line[1]
        if point1[0] == point2[0]:
            increment: int = 1
            if point2[1] < point1[1]:
                increment = -1
            for y_num in range(point1[1], point2[1] + increment, increment):
                dict_key: Tuple[int, int] = (point1[0], y_num)
                if dict_key not in line_cover_map.keys():
                    line_cover_map[dict_key] = 0
                line_cover_map[dict_key] += 1

        elif point1[1] == point2[1]:
            increment = 1
            if point2[0] < point1[0]:
                increment = -1
            for x_num in range(point1[0], point2[0] + increment, increment):
                dict_key = (x_num, point1[1])
                if dict_key not in line_cover_map.keys():
                    line_cover_map[dict_key] = 0
                line_cover_map[dict_key] += 1
        else:
            if process_diag:
                x_increment: int = 1
                if point2[0] < point1[0]:
                    x_increment = -1
                y_increment: int = 1
                if point2[1] < point1[1]:
                    y_increment = -1

                cur_point: Tuple[int, int] = point1
                if cur_point not in line_cover_map.keys():
                    line_cover_map[cur_point] = 0
                line_cover_map[cur_point] += 1

                while cur_point != point2:
                    cur_point = (cur_point[0] + x_increment,
                                 cur_point[1] + y_increment)
                    if cur_point not in line_cover_map.keys():
                        line_cover_map[cur_point] = 0
                    line_cover_map[cur_point] += 1

    print_overlap(line_cover_map)

    count_multiple_overlap = 0
    for index, item in line_cover_map.items():
        if item > 1:
            count_multiple_overlap += 1

    return count_multiple_overlap


def solve(puzzle_input: str):
    """Solve the puzzle for the given input"""
    data = parse(puzzle_input)
    solution1 = compute(data, False)
    solution2 = compute(data, True)
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
