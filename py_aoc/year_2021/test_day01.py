import pytest

from py_aoc.year_2021.day01 import INPUT_S, INPUT_GITHUB, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return parse(f.read())


def test_parse(example):
    example == [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]


def test_part1_example(example) -> None:
    assert compute(example, 1) == 7


def test_part1_github(github) -> None:
    assert compute(github, 1) == 1448


def test_part2_example(example) -> None:
    assert compute(example, 3) == 5


def test_part2_github(github) -> None:
    assert compute(github, 3) == 1471
