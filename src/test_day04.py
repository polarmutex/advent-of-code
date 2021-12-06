import pytest

from src.day04 import INPUT_S, INPUT_TXT, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return f.read()


def test_part1_example(example) -> None:
    assert part1(example) == 4512


def test_part1_input(input) -> None:
    assert part1(input) == 46920


def test_part2_example(example) -> None:
    assert part2(example) == 1924


def test_part2_input(input) -> None:
    assert part2(input) == 12635
