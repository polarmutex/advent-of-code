import pytest

from src.day02 import INPUT_S, INPUT_TXT, parse, part1, part2


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return parse(f.read())


def test_parse(example):
    example == [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]


def test_part1_example(example) -> None:
    assert part1(example) == 150


def test_part1_input(input) -> None:
    assert part1(input) == 1250395


def test_part2_example(example) -> None:
    assert part2(example) == 900


def test_part2_input(input) -> None:
    assert part2(input) == 1451210346
