import pytest

from py_aoc.year_2021.day07 import INPUT_S, parse, INPUT_GITHUB, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


def test_parse(example):
    assert parse(example) == [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]


def test_part1_example(example) -> None:
    assert part1(example) == 37


def test_part2_example(example) -> None:
    assert part2(example) == 168


def test_part1_github(github) -> None:
    assert part1(github) == 335330


def test_part2_github(github) -> None:
    assert part2(github) == 92439766
