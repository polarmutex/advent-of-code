import pytest

from py_aoc.year_2021.day14 import INPUT_S, parse, INPUT_GITHUB, INPUT_GOOGLE, INPUT_TWITTER, INPUT_REDDIT, part1, part2


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
def test_parse(example):
    actual = []  # parse(example)
    expected = [
    ]
    print(actual)
    assert actual == expected


def test_part1_example(example) -> None:
    assert part1(example) == 1588


def test_part2_example(example) -> None:
    assert part2(example) == 2188189693529


def test_part1_github(github) -> None:
    assert part1(github) == 3831


def test_part2_github(github) -> None:
    assert part2(github) == 5725739914282


def test_part1_google(google) -> None:
    assert part1(google) == 2068


def test_part2_google(google) -> None:
    assert part2(google) == 2158894777814


def test_part1_twitter(twitter) -> None:
    assert part1(twitter) == 3095


def test_part2_twitter(twitter) -> None:
    assert part2(twitter) == 3152788426516


def test_part1_reddit(reddit) -> None:
    assert part1(reddit) == 3247


@pytest.mark.skip(reason="Not implemented")
def test_part2_reddit(reddit) -> None:
    assert part2(reddit) == 4110568157153
