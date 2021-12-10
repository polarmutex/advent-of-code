import pytest

from py_aoc.year_2021.day05 import INPUT_S, INPUT_GITHUB, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
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


def test_part1_github(github) -> None:
    assert compute(github, False) == 7269


def test_part2_example(example) -> None:
    assert compute(example, True) == 12


def test_part2_github(github) -> None:
    assert compute(github, True) == 21140
