import pytest

from src.advent_of_code.day06 import INPUT_S, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


def test_parse(example):
    assert example == [3, 4, 3, 1, 2]


def test_part1(example) -> None:
    assert compute(example, 80) == 5934


def test_part2(example) -> None:
    assert compute(example, 256) == 26984457539
