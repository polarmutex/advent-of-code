import pytest

from py_aoc.year_2021.day05 import INPUT_S, INPUT_TXT, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return parse(f.read())


def test_parse(example):
    assert example == [
        ((0, 9), (5, 9)),
        ((8, 0), (0, 8)),
        ((9, 4), (3, 4)),
        ((2, 2), (2, 1)),
        ((7, 0), (7, 4)),
        ((6, 4), (2, 0)),
        ((0, 9), (2, 9)),
        ((3, 4), (1, 4)),
        ((0, 0), (8, 8)),
        ((5, 5), (8, 2)),
    ]


def test_part1_example(example) -> None:
    assert compute(example, False) == 5


def test_part1_input(input) -> None:
    assert compute(input, False) == 7269


def test_part2_example(example) -> None:
    assert compute(example, True) == 12


def test_part2_input(input) -> None:
    assert compute(input, True) == 21140
