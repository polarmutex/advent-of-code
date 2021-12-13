from __future__ import annotations

import argparse
import os.path
import collections
from typing import List, Tuple, Generator, Set, Dict

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day13_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day13_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day13_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day13_twitter.txt")
INPUT_S = """\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"""

Paper = Set
Fold = Tuple[str, int]


def parse(input: str):
    map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def fold_y(paper_to_fold: Paper, fold: Fold) -> Paper:
    new_paper: Paper = set()

    for x, y in paper_to_fold:
        if y > fold[1]:
            new_y = fold[1] - (y - fold[1])
            new_paper.add((x, new_y))
        else:
            new_paper.add((x, y))

    return new_paper


def fold_x(paper_to_fold: Paper, fold: Fold) -> Paper:
    new_paper: Paper = set()

    for x, y in paper_to_fold:
        if x > fold[1]:
            new_x = fold[1] - (x - fold[1])
            new_paper.add((new_x, y))
        else:
            new_paper.add((x, y))

    return new_paper


def print_code(paper: Paper):
    max_x = max(x for x, _ in paper)
    max_y = max(y for _, y in paper)

    result = ""
    for y in range(0, max_y + 1):
        for x in range(0, max_x + 1):
            if (x, y) in paper:
                result += "#"
            else:
                result += " "
        result += "\n"
    print(result)


def part1(input: str) -> int:

    paper: Paper = set()
    folds: List[Fold] = []

    lines = input.splitlines()
    get_points = True
    for line in lines:
        if line == "":
            get_points = False
            continue
        if get_points:
            x, y = line.split(',')
            paper.add((int(x), int(y)))
        else:
            axis, num = line.replace("fold along ", "").split('=')
            folds.append((axis, int(num)))

    if folds[0][0] == 'x':
        cur_paper = fold_x(paper, folds[0])
    elif folds[0][0] == 'y':
        cur_paper = fold_y(paper, folds[0])
    else:
        print("err")

    return len(cur_paper)


def part2(input: str):
    paper: Paper = set()
    folds: List[Fold] = []

    lines = input.splitlines()
    get_points = True
    for line in lines:
        if line == "":
            get_points = False
            continue
        if get_points:
            x, y = line.split(',')
            paper.add((int(x), int(y)))
        else:
            axis, num = line.replace("fold along ", "").split('=')
            folds.append((axis, int(num)))

    cur_paper = paper
    for fold in folds:
        if fold[0] == 'x':
            cur_paper = fold_x(cur_paper, fold)
        elif fold[0] == 'y':
            cur_paper = fold_y(cur_paper, fold)
        else:
            print("err")

    print_code(cur_paper)

    return 0


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
