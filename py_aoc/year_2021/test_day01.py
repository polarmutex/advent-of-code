import pytest

from py_aoc.year_2021.day01 import INPUT_S, INPUT_TXT, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return parse(f.read())


def test_parse(example):
    example == [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]


def test_part1_example(example) -> None:
    assert compute(example, 1) == 7


def test_part1_input(input) -> None:
    assert compute(input, 1) == 1448


def test_part2_example(example) -> None:
    assert compute(example, 3) == 5


def test_part2_input(input) -> None:
    assert compute(input, 3) == 1471
