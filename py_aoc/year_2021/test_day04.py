import pytest

from py_aoc.year_2021.day04 import INPUT_S, INPUT_GITHUB, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


def test_part1_example(example) -> None:
    assert part1(example) == 4512


def test_part1_github(github) -> None:
    assert part1(github) == 46920


def test_part2_example(example) -> None:
    assert part2(example) == 1924


def test_part2_github(github) -> None:
    assert part2(github) == 12635
