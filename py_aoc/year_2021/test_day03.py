import pytest

from py_aoc.year_2021.day03 import INPUT_S, INPUT_GITHUB, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


def test_part1_example(example) -> None:
    assert part1(example) == 198


def test_part1_github(github) -> None:
    assert part1(github) == 845186


def test_part2_example(example) -> None:
    assert part2(example) == 230


def test_part2_github(github) -> None:
    assert part2(github) == 4636702
