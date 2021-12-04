from __future__ import annotations

import argparse
import os.path

import pytest

from support import timing
from typing import List

INPUT_TXT = os.path.join(os.path.dirname(__file__), "input.txt")


class Board:
    def __init__(self):
        self.board: List[List[int]] = []
        self.marks: List[List[bool]] = []

    def add_row(self, row: List[int]):
        self.board.append(row)
        array: List[bool] = []
        for i in range(len(row)):
            array.append(False)
        self.marks.append(array)

    def mark_number(self, called_num: int):
        for yi, y in enumerate(self.board):
            for xi, x in enumerate(y):
                if x == called_num:
                    self.marks[yi][xi] = True

    def check_win(self) -> bool:
        result: bool = True

        for y in self.marks:
            result = True
            for x in y:
                result = result and x
            if result is True:
                return True

        for xi in range(len(self.marks[0])):
            result = True
            for yi in range(len(self.marks)):
                result = result and self.marks[yi][xi]
            if result is True:
                return True

        return False

    def sum_unmarked(self) -> int:
        sum: int = 0
        for yi, y in enumerate(self.marks):
            for xi, x in enumerate(y):
                if x is False:
                    sum += self.board[yi][xi]
        return sum

    def print(self):
        for yi, y in enumerate(self.board):
            print(y)
        for yi, y in enumerate(self.marks):
            print(y)


def compute(s: str) -> int:
    lines = s.splitlines()
    draw_numbers: List[int] = []
    current_board: Board = Board()
    boards: List[Board] = []

    for index, line in enumerate(lines):

        print("---" + line + "---")

        if index == 0:
            for num_str in line.split(','):
                draw_numbers.append(int(num_str))
        elif index == 1:
            pass
        else:
            if line == "":
                boards.append(current_board)
                current_board = Board()
            else:
                num_array: List[int] = []
                for num_str in line.split():
                    num_array.append(int(num_str))
                current_board.add_row(num_array)

    boards.append(current_board)

    print("draw numbers are: ", draw_numbers)
    for board in boards:
        board.print()
    print("-----------------------")

    last_board: bool = False
    for current_called_num in draw_numbers:

        for board in boards:
            board.mark_number(current_called_num)

        temp: List[Board] = []
        for board in boards:
            if not board.check_win():
                temp.append(board)
            if last_board and board.check_win():
                sum = boards[0].sum_unmarked()
                print(sum)
                print(current_called_num)
                return current_called_num * sum
        boards = temp

        if len(boards) == 1:
            last_board = True

    # Error
    return 0


INPUT_S = """\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""


@ pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 1924),),
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
