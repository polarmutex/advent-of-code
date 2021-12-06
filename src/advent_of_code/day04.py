from __future__ import annotations

import argparse
import os.path

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


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_TXT)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        print(compute(f.read()))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
