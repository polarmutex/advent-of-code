import pytest

from py_aoc.year_2021.day06 import INPUT_S, parse, INPUT_GITHUB, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


def test_parse(example):
    assert parse(example) == [3, 4, 3, 1, 2]


def test_part1_example(example) -> None:
    assert part1(example) == 5934


def test_part2_example(example) -> None:
    assert part2(example) == 26984457539


def test_part1_github(github) -> None:
    assert part1(github) == 388739


def test_part2_github(github) -> None:
    assert part2(github) == 1741362314973
