import pytest

from py_aoc.year_2021.day09 import INPUT_S, parse, INPUT_GITHUB, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


def test_parse(example):
    actual = parse(example)
    expected = [
        [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ]
    print(actual)
    assert actual == expected


def test_part1_example(example) -> None:
    assert part1(example) == 15


def test_part2_example(example) -> None:
    assert part2(example) == 1134


def test_part1_github(github) -> None:
    assert part1(github) == 524


def test_part2_github(github) -> None:
    assert part2(github) == 1235430
