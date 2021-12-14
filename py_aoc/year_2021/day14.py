from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day14_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day14_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day14_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day14_twitter.txt")
INPUT_S = """\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"""

PolymerTemplate = str
PairInsertionRules = Dict[str, str]


def parse(input: str):
    map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str):
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def compute(input: str, steps: int) -> int:
    start_template: PolymerTemplate = ""
    pair_insertion_rules: PairInsertionRules = {}

    lines = input.splitlines()
    start_template = lines[0]

    for line in lines[2:]:
        left, right = line.split(' -> ')
        pair_insertion_rules[left] = right

    bigrams: Counter[str] = Counter()
    counts: Counter[str] = Counter()

    for index, char in enumerate(start_template):
        if index != len(start_template) - 1:
            bigram = f"{char}{start_template[index+1]}"
            bigrams[bigram] += 1

    for s in range(steps):
        print(f"step: {s}", bigrams)
        next_bigrams = bigrams.copy()
        for bigram in bigrams:
            if bigrams[bigram] > 0:
                insert_char = pair_insertion_rules[bigram]
                left_bigram = f"{bigram[0]}{insert_char}"
                right_bigram = f"{insert_char}{bigram[1]}"
                next_bigrams[left_bigram] += bigrams[bigram]
                next_bigrams[right_bigram] += bigrams[bigram]
                next_bigrams[bigram] -= bigrams[bigram]
        bigrams = next_bigrams

    for bigram in bigrams:
        counts[bigram[0]] += bigrams[bigram]
        counts[bigram[1]] += bigrams[bigram]

    mc = counts.most_common()[0]
    mc_count = mc[1]
    lc = counts.most_common()[-1]
    lc_count = lc[1]

    mc_lc_diff = int((mc_count-lc_count)/2)
    print((mc_count-lc_count)/2)
    print(start_template)
    print(f"s: {start_template[0]} e: {start_template[-1]}")
    print(f"mc: {mc[0]} lc: {lc[0]}")
    if start_template[0] == mc[0]:
        mc_count += 1
    if start_template[-1] == mc[0]:
        mc_count += 1
    if start_template[0] == lc[0]:
        lc_count += 1
    if start_template[-1] == lc[0]:
        lc_count += 1

    mc_lc_diff = int((mc_count-lc_count)/2)
    return mc_lc_diff


def part1(input: str) -> int:
    return compute(input, 10)


def part2(input: str):
    return compute(input, 40)


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
