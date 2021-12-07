import pytest

from src.day06 import INPUT_S, parse, INPUT_TXT, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return f.read()


def test_parse(example):
    assert parse(example) == [3, 4, 3, 1, 2]


def test_part1_example(example) -> None:
    assert part1(example) == 5934


def test_part2_example(example) -> None:
    assert part2(example) == 26984457539


def test_part1_input(input) -> None:
    assert part1(input) == 388739


def test_part2_input(input) -> None:
    assert part2(input) == 1741362314973
