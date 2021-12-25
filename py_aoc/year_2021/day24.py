from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
import heapq
from copy import deepcopy
import z3

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day24_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day24_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day24_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day24_twitter.txt")
INPUT_S = """\
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
"""

Instructions = str
ALU_Program = List[Instructions]


def parse(input: str) -> ALU_Program:
    instructions: List[Instructions] = []
    for index, line in enumerate(input.splitlines()):
        instructions.append(line)
    return instructions


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


class ALU:
    def __init__(self) -> None:
        self.input: List[int] = []
        self.registers: Dict[str, int] = {
            'w': 0,
            'x': 0,
            'y': 0,
            'z': 0,
        }

    def clear(self):
        self.input = []
        self.registers = {
            'w': 0,
            'x': 0,
            'y': 0,
            'z': 0,
        }

    def load_input(self, input: List[int]):
        self.input = input

    def execute(self, input: List[Instructions]) -> None:
        for instruction in input:
            parts = instruction.split()
            command = parts[0]
            if len(parts) == 2:
                a = parts[1]
                b = None
            elif len(parts) == 3:
                a = parts[1]
                b = parts[2]

            if a not in ['w', 'x', 'y', 'z']:
                continue

            if command == "inp":
                print(f"\t{command} - {a} - {b}")
                self.registers[a] = self.input.pop()
            elif command == "add" and b is not None:
                print(f"\t{command} - {a} - {b}")
                if b in ['w', 'x', 'y', 'z']:
                    self.registers[a] = self.registers[a] + self.registers[b]
                else:
                    self.registers[a] = self.registers[a] + int(b)
            elif command == "mul" and b is not None:
                print(f"\t{command} - {a} - {b}")
                if b in ['w', 'x', 'y', 'z']:
                    self.registers[a] = self.registers[a] * self.registers[b]
                else:
                    self.registers[a] = self.registers[a] * int(b)
            elif command == "div" and b is not None:
                print(f"\t{command} - {a} - {b}")
                if b in ['w', 'x', 'y', 'z']:
                    if self.registers[b] == 0:
                        continue
                    self.registers[a] = int(self.registers[a] /
                                            self.registers[b])
                else:
                    if int(b) == 0:
                        continue
                    self.registers[a] = int(self.registers[a] / int(b))
            elif command == "mod" and b is not None:
                print(f"\t{command} - {a} - {b}")
                if b in ['w', 'x', 'y', 'z']:
                    if self.registers[a] < 0 or self.registers[b] <= 0:
                        continue
                    self.registers[a] = int(self.registers[a] %
                                            self.registers[b])
                else:
                    if self.registers[a] < 0 or int(b) <= 0:
                        continue
                    self.registers[a] = int(self.registers[a] % int(b))
            elif command == "eql" and b is not None:
                print(f"\t{command} - {a} - {b}")
                if b in ['w', 'x', 'y', 'z']:
                    self.registers[a] = int(self.registers[a] ==
                                            self.registers[b])
                else:
                    self.registers[a] = int(self.registers[a] == int(b))


def next_model_number(model_num: str) -> str:
    temp: int = int(model_num) - 1
    while("0" in str(temp)):
        temp -= 1
    return str(temp)


class ALUZ3:
    def __init__(self) -> None:
        self.zero = z3.BitVecVal(0, 64)
        self.one = z3.BitVecVal(1, 64)
        self.registers: Dict[str, int] = {
            'w': self.zero,
            'x': self.zero,
            'y': self.zero,
            'z': self.zero,
        }
        self.solver = z3.Optimize()
        self.inputs: List[z3.Int] = [
            z3.BitVec(f'd_{i}', 64) for i in range(14)]
        self.inputs_iter = iter(self.inputs)
        for digit in self.inputs:
            self.solver.add(1 <= digit)
            self.solver.add(digit <= 9)
        self.next_input = -1

    def execute(self, input: List[Instructions]) -> Tuple[int, int]:
        for idx, instruction in enumerate(input):
            parts = instruction.split()
            command = parts[0]
            if len(parts) == 2:
                a = parts[1]
                b = None
            elif len(parts) == 3:
                a = parts[1]
                b = parts[2]

            if a not in ['w', 'x', 'y', 'z']:
                raise AssertionError("Illegal a value")

            c = z3.BitVec(f'v_{idx}', 64)
            if command == "inp":
                self.registers[a] = next(self.inputs_iter)
                continue
            elif command == "add" and b is not None:
                if b in ['w', 'x', 'y', 'z']:
                    self.solver.add(c == self.registers[a] + self.registers[b])
                else:
                    self.solver.add(c == self.registers[a] + int(b))
            elif command == "mul" and b is not None:
                if b in ['w', 'x', 'y', 'z']:
                    self.solver.add(c == self.registers[a] * self.registers[b])
                else:
                    self.solver.add(c == self.registers[a] * int(b))
            elif command == "div" and b is not None:
                if b in ['w', 'x', 'y', 'z']:
                    self.solver.add(self.registers[b] > 0)
                    self.solver.add(
                        c == self.registers[a] // self.registers[b])
                else:
                    self.solver.add(int(b) > 0)
                    self.solver.add(c == self.registers[a] / int(b))
            elif command == "mod" and b is not None:
                if b in ['w', 'x', 'y', 'z']:
                    self.solver.add(self.registers[a] >= 0)
                    self.solver.add(self.registers[b] > 0)
                    self.solver.add(
                        c == self.registers[a] % self.registers[b])
                else:
                    self.solver.add(self.registers[a] >= 0)
                    self.solver.add(int(b) > 0)
                    self.solver.add(c == self.registers[a] % int(b))
            elif command == "eql" and b is not None:
                if b in ['w', 'x', 'y', 'z']:
                    self.solver.add(
                        c == z3.If(
                            self.registers[a] == self.registers[b], self.one, self.zero))
                else:
                    self.solver.add(c == z3.If(
                        self.registers[a] == int(b), self.one, self.zero))
            self.registers[a] = c
        self.solver.add(self.registers['z'] == 0)

        min: int = 0
        self.solver.push()
        self.solver.minimize(
            sum((10 ** i) * d for i, d in enumerate(self.inputs[::-1])))
        print(self.solver.check())
        m = self.solver.model()
        min = int(''.join([str(m[d]) for d in self.inputs]))
        self.solver.pop()

        max: int = 0
        self.solver.push()
        self.solver.maximize(
            sum((10 ** i) * d for i, d in enumerate(self.inputs[::-1])))
        print(self.solver.check())
        m = self.solver.model()
        max = int(''.join([str(m[d]) for d in self.inputs]))
        self.solver.pop()

        return min, max


def part1(input: str) -> int:
    insts = parse(input)
    alu = ALUZ3()
    min, max = alu.execute(insts)
    return max


def part2(input: str) -> int:
    insts = parse(input)
    alu = ALUZ3()
    min, max = alu.execute(insts)
    return min


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
