import pytest

from py_aoc.year_2021.day07 import INPUT_S, parse, INPUT_TXT, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return f.read()


def test_parse(example):
    assert parse(example) == [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]


def test_part1_example(example) -> None:
    assert part1(example) == 37


def test_part2_example(example) -> None:
    assert part2(example) == 168


@pytest.mark.skip(reason="Not implemented")
def test_part1_input(input) -> None:
    assert part1(input) == 388739


@pytest.mark.skip(reason="Not implemented")
def test_part2_input(input) -> None:
    assert part2(input) == 1741362314973
