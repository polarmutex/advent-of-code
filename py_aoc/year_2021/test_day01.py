import pytest

from py_aoc.year_2021.day01 import INPUT_S, INPUT_GITHUB, INPUT_GOOGLE, INPUT_REDDIT, INPUT_TWITTER, compute, parse


@pytest.fixture
def example():
    return parse(INPUT_S)


@pytest.fixture
def github():
    with open(INPUT_GITHUB) as f:
        return parse(f.read())


@pytest.fixture
def google():
    with open(INPUT_GOOGLE) as f:
        return parse(f.read())


@pytest.fixture
def twitter():
    with open(INPUT_TWITTER) as f:
        return parse(f.read())


@pytest.fixture
def reddit():
    with open(INPUT_REDDIT) as f:
        return parse(f.read())


def test_parse(example):
    example == [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]


def test_part1_example(example) -> None:
    assert compute(example, 1) == 7


def test_part1_github(github) -> None:
    assert compute(github, 1) == 1448


def test_part1_google(google) -> None:
    assert compute(google, 1) == 1121


def test_part1_twitter(twitter) -> None:
    assert compute(twitter, 1) == 1557


def test_part1_reddit(reddit) -> None:
    assert compute(reddit, 1) == 1583


def test_part2_example(example) -> None:
    assert compute(example, 3) == 5


def test_part2_github(github) -> None:
    assert compute(github, 3) == 1471


def test_part2_google(google) -> None:
    assert compute(google, 3) == 1065


def test_part2_twitter(twitter) -> None:
    assert compute(twitter, 3) == 1608


def test_part2_reddit(reddit) -> None:
    assert compute(reddit, 3) == 1627
