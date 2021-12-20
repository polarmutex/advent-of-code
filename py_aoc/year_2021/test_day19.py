import pytest

from py_aoc.year_2021.day19 import INPUT_S, parse, INPUT_GITHUB, INPUT_GOOGLE, INPUT_TWITTER, INPUT_REDDIT, part1, part2
from typing import List, Any


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return f.read()


@pytest.fixture
def google():
    with open(INPUT_GOOGLE) as f:
        return f.read()


@pytest.fixture
def twitter():
    with open(INPUT_TWITTER) as f:
        return f.read()


@pytest.fixture
def reddit():
    with open(INPUT_REDDIT) as f:
        return f.read()


@pytest.mark.skip(reason="Not implemented")
def test_parse(example: str) -> None:
    actual: List[Any] = []  # parse(example)
    expected: List[Any] = [
    ]
    print(actual)
    assert actual == expected


def test_part1_example(example: str) -> None:
    assert(part1(example) == 79)


def test_part2_example(example: str) -> None:
    assert(part2(example) == 3621)


def test_part1_github(github: str) -> None:
    assert part1(github) == 350


def test_part2_github(github: str) -> None:
    assert part2(github) == 10895


def test_part1_google(google: str) -> None:
    assert part1(google) == 326


def test_part2_google(google: str) -> None:
    assert part2(google) == 10630


def test_part1_twitter(twitter: str) -> None:
    assert part1(twitter) == 338


def test_part2_twitter(twitter: str) -> None:
    assert part2(twitter) == 9862


def test_part1_reddit(reddit: str) -> None:
    assert part1(reddit) == 320


def test_part2_reddit(reddit: str) -> None:
    assert part2(reddit) == 9655
