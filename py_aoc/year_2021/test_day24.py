import pytest

from py_aoc.year_2021.day24 import INPUT_S, parse, INPUT_GITHUB, INPUT_GOOGLE, INPUT_TWITTER, INPUT_REDDIT, part1, part2
from typing import List, Any


@pytest.fixture
def example() -> str:
    return INPUT_S


@pytest.fixture
def github() -> str:
    with open(INPUT_GITHUB) as f:
        return f.read()


@pytest.fixture
def google() -> str:
    with open(INPUT_GOOGLE) as f:
        return f.read()


@pytest.fixture
def twitter() -> str:
    with open(INPUT_TWITTER) as f:
        return f.read()


@pytest.fixture
def reddit() -> str:
    with open(INPUT_REDDIT) as f:
        return f.read()


@pytest.mark.skip(reason="Not implemented")
def test_parse(example: str) -> None:
    actual: List[Any] = []  # parse(example)
    expected: List[Any] = [
    ]
    print(actual)
    assert actual == expected


@pytest.mark.skip(reason="Not implemented")
def test_part1_example(example: str) -> None:
    assert(part1(example) == 12521)


@pytest.mark.skip(reason="Not implemented")
def test_part2_example(example: str) -> None:
    assert(part2(example) == 44169)


def test_part1_github(github: str) -> None:
    assert part1(github) == 94399898949959


def test_part2_github(github: str) -> None:
    assert part2(github) == 21176121611511


def test_part1_google(google: str) -> None:
    assert part1(google) == 99995969919326


def test_part2_google(google: str) -> None:
    assert part2(google) == 48111514719111


def test_part1_twitter(twitter: str) -> None:
    assert part1(twitter) == 93499629698999


def test_part2_twitter(twitter: str) -> None:
    assert part2(twitter) == 11164118121471


def test_part1_reddit(reddit: str) -> None:
    assert part1(reddit) == 99893999291967


def test_part2_reddit(reddit: str) -> None:
    assert part2(reddit) == 34171911181211
