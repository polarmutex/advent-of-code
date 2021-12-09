from __future__ import annotations

import numpy
import argparse
import os.path
from typing import List, Tuple, Dict, Optional

from support import timing

INPUT_TXT = os.path.join("data", "2021", "day09_github.txt")
INPUT_S = """\
2199943210
3987894921
9856789892
8767896789
9899965678
"""

HeightMap = List[List[int]]


def parse(input: str) -> HeightMap:
    map: HeightMap = []
    for index, line in enumerate(input.splitlines()):
        map.append([])
        for char in line:
            map[index].append(int(char))
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def is_low_point(row: int, col: int, data: HeightMap):
    all_lower: bool = True
    val = data[row][col]

    down: int = val
    if row != len(data)-1:
        down = data[row+1][col]
        all_lower = all_lower and (val < down)
    up: int = val
    if row != 0:
        up = data[row-1][col]
        all_lower = all_lower and (val < up)
    left: int = val
    if col != 0:
        left = data[row][col-1]
        all_lower = all_lower and (val < left)
    right: int = val
    if col != len(data[row])-1:
        right = data[row][col+1]
        all_lower = all_lower and (val < right)

    return all_lower


def part1(input: str) -> int:
    data: HeightMap = parse(input)

    sum_risk = 0
    for row, row_data in enumerate(data):
        for col, val in enumerate(row_data):
            if is_low_point(row, col, data):
                sum_risk += 1 + val

    return sum_risk


def part2(input: str):
    data: HeightMap = parse(input)

    # Assumption single low point

    def search_basin(row: int, col: int, visited=None) -> List[Tuple[int, int]]:
        # Depth First Search

        if visited is None:
            visited = []
        print("visiting row: " + str(row) + " col:" + str(col))
        visited.append((row, col))
        val = data[row][col]

        return visited

    basins: List[int] = []
    for row, row_data in enumerate(data):
        for col, val in enumerate(row_data):
            if is_low_point(row, col, data):
                queue = []
                visited = []
                visited.append((row, col))
                queue.append((row, col))
                while len(queue) != 0:
                    n = queue.pop(0)
                    cur_row: int = n[0]
                    cur_col: int = n[1]
                    print("visiting row: " + str(cur_row) +
                          " col:" + str(cur_col))

                    down: int | None = None
                    if cur_row != len(data)-1:
                        down = data[cur_row+1][cur_col]
                    up: int | None = None
                    if cur_row != 0:
                        up = data[cur_row-1][cur_col]
                    left: int | None = None
                    if cur_col != 0:
                        left = data[cur_row][cur_col-1]
                    right: int | None = None
                    if cur_col != len(data[cur_row])-1:
                        right = data[cur_row][cur_col+1]

                    if down is not None:
                        if val < down and down != 9:
                            if (cur_row+1, cur_col) not in visited:
                                visited.append((cur_row+1, cur_col))
                                queue.append((cur_row+1, cur_col))
                    else:
                        print("no down")
                    if up is not None:
                        if val < up and up != 9:
                            if (cur_row-1, cur_col) not in visited:
                                visited.append((cur_row-1, cur_col))
                                queue.append((cur_row-1, cur_col))
                    else:
                        print("no up")
                    if left is not None:
                        if val < left and left != 9:
                            if (cur_row, cur_col-1) not in visited:
                                visited.append((cur_row, cur_col-1))
                                queue.append((cur_row, cur_col-1))
                    else:
                        print("no left")
                    if right is not None:
                        if val < right and right != 9:
                            if (cur_row, cur_col+1) not in visited:
                                visited.append((cur_row, cur_col+1))
                                queue.append((cur_row, cur_col+1))
                    else:
                        print("no right")
                print(visited)
                basins.append(len(visited))
                if len(basins) > 3:
                    basins.remove(sorted(basins)[0])

    print(basins)
    result: int = 1
    for item in basins:
        result *= item

    return result


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
