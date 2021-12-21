from __future__ import annotations

import argparse
import copy
import os.path
import math
import collections
import functools
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple, Any
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray
from enum import Enum

from support import timing

INPUT_GOOGLE = os.path.join("data", "2021", "day21_google.txt")
INPUT_GITHUB = os.path.join("data", "2021", "day21_github.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day21_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day21_twitter.txt")
INPUT_S = """\
Player 1 starting position: 4
Player 2 starting position: 8
"""


class Player(NamedTuple):
    id: int = 0
    start_num: int = 0


def parse(input: str) -> Tuple[Player, Player]:
    temp = input.splitlines()
    player1: Player = Player(id=1, start_num=int(temp[0].split(':')[1]))
    player2: Player = Player(id=1, start_num=int(temp[1].split(':')[1]))
    return player1, player2


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def next_pos(i: int) -> int:
    return (i - 1) % 10 + 1


def next_dice(i: int) -> int:
    return (i - 1) % 100 + 1


def dirac_dice(player1: Player, player2: Player) -> Tuple[Tuple[int, int], int, int]:
    dice: Tuple[int, int] = (1, 0)

    player1_score: int = 0
    player2_score: int = 0

    player1_pos: int = player1.start_num
    player2_pos: int = player2.start_num
    print(f"player 1 starts at {player1_pos}")
    print(f"player 2 starts at {player2_pos}")

    while True:
        player1_pos = next_pos(player1_pos + dice[0] + dice[0] + 1 + dice[0]+2)
        dice = (next_dice(dice[0] + 3), dice[1] + 3)
        player1_score += player1_pos
        print(
            f"Player 1 rolls and moves to space {player1_pos} for a total score of {player1_score}")
        print(f"next dice start: {dice[0]}")
        if player1_score >= 1000:
            break

        player2_pos = next_pos(player2_pos + dice[0] + dice[0] + 1 + dice[0]+2)
        dice = (next_dice(dice[0] + 3), dice[1] + 3)
        player2_score += player2_pos
        print(
            f"Player 2 rolls and moves to space {player2_pos} for a total score of {player2_score}")
        if player2_score >= 1000:
            break
    return dice, player1_score, player2_score


def part1(input: str) -> int:
    player1, player2 = parse(input)
    dice, p1_score, p2_score = dirac_dice(player1, player2)
    return dice[1] * (p1_score if p1_score < 1000 else p2_score)


# In one quantum turn there will be 27 universes generated in the following states
quantum_dice_rolls = collections.Counter(
    i + j + k for i in (1, 2, 3) for j in (1, 2, 3) for k in (1, 2, 3))


@functools.lru_cache(maxsize=None)
def win_count(
        cur_player_pos: int,
        cur_player_score: int,
        other_player_pos: int,
        other_player_score: int) -> Tuple[int, int]:
    cur_player_wins = 0
    other_player_wins = 0
    # Loop over 27 states that will be spawned
    for roll_total, count in quantum_dice_rolls.items():
        # update the game for this roll
        updated_cur_player_pos = next_pos(cur_player_pos + roll_total)
        updated_cur_player_score = cur_player_score + updated_cur_player_pos
        if updated_cur_player_score >= 21:
            cur_player_wins += count
        else:
            # spawn the next universe
            future_other_player_wins, future_cur_player_wins = win_count(
                other_player_pos,
                other_player_score,
                updated_cur_player_pos,
                updated_cur_player_score)
            cur_player_wins += future_cur_player_wins * count
            other_player_wins += future_other_player_wins * count
    return cur_player_wins, other_player_wins


def part2(input: str) -> int:
    player1, player2 = parse(input)
    player1_wins, player2_wins = win_count(
        player1.start_num, 0, player2.start_num, 0)
    return max(player1_wins, player2_wins)


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
