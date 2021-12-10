from __future__ import annotations

import argparse
import os.path
from typing import List, Tuple

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day10_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day10_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day10_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day10_twitter.txt")
INPUT_S = """\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"""

NavLines = List[str]


def parse(input: str) -> NavLines:
    nav_lines: NavLines = []
    for index, line in enumerate(input.splitlines()):
        nav_lines.append(line)
    return nav_lines


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def compile_line(line: str) -> Tuple[bool, str, str]:
    queue: List[str] = []

    for char in line:

        if char == '(':
            queue.append('(')
        elif char == '[':
            queue.append('[')
        elif char == '{':
            queue.append('{')
        elif char == '<':
            queue.append('<')

        elif char == ')':
            cl = queue.pop(-1)
            if cl != "(":
                print("Expected " + cl + ", but found " + char)
                return False, ")", ""
        elif char == ']':
            cl = queue.pop(-1)
            if cl != "[":
                print("Expected " + cl + ", but found " + char)
                return False, "]", ""
        elif char == '}':
            cl = queue.pop(-1)
            if cl != "{":
                print("Expected " + cl + ", but found " + char)
                return False, "}", ""
        elif char == '>':
            cl = queue.pop(-1)
            if cl != "<":
                print("Expected " + cl + ", but found " + char)
                return False, ">", ""
        else:
            print("ERR: unknown " + char)

    missing = ""
    while(len(queue) != 0):
        missing += queue.pop(-1)

    return True, "", missing


def part1(input: str) -> int:
    data: NavLines = parse(input)
    points: int = 0

    for line in data:
        res, char, _ = compile_line(line)
        if not res:
            if char == ")":
                points += 3
            elif char == "]":
                points += 57
            elif char == "}":
                points += 1197
            elif char == ">":
                points += 25137
            else:
                print("ERR in points " + char)

    return points


def part2(input: str):
    data: NavLines = parse(input)
    scores: List[int] = []
    for line in data:
        res, char, missing = compile_line(line)
        if res and missing != "":
            score: int = 0
            for char in missing:
                score *= 5
                if char == "(":
                    score += 1
                elif char == "[":
                    score += 2
                elif char == "{":
                    score += 3
                elif char == "<":
                    score += 4
                else:
                    print("ERR in points " + char)
            scores.append(score)

    scores.sort()
    middle_score: int = scores[int(len(scores)/2)]
    return middle_score


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
