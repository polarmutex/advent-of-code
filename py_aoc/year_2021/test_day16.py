import pytest

from py_aoc.year_2021.day16 import INPUT_S, INPUT_S2, parse, INPUT_GITHUB, INPUT_GOOGLE, INPUT_TWITTER, INPUT_REDDIT, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def example2():
    return INPUT_S2


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


def test_part1_example(example: str) -> None:
    lines = example.splitlines()

    assert(part1(lines[0]) == 6)
    assert(part1(lines[1]) == 9)
    assert(part1(lines[2]) == 14)
    assert(part1(lines[3]) == 16)
    assert(part1(lines[4]) == 12)
    assert(part1(lines[5]) == 23)
    assert(part1(lines[6]) == 31)


def test_part2_example(example2: str) -> None:
    lines = example2.splitlines()

    assert(part2(lines[0]) == 3)
    assert(part2(lines[1]) == 54)
    assert(part2(lines[2]) == 7)
    assert(part2(lines[3]) == 9)
    assert(part2(lines[4]) == 1)
    assert(part2(lines[5]) == 0)
    assert(part2(lines[6]) == 0)
    assert(part2(lines[6]) == 0)


def test_part1_github(github) -> None:
    assert part1(github) == 925


def test_part2_github(github) -> None:
    assert part2(github) == 342997120375


def test_part1_google(google) -> None:
    assert part1(google) == 1002


def test_part2_google(google) -> None:
    assert part2(google) == 1673210814091


def test_part1_twitter(twitter) -> None:
    assert part1(twitter) == 1012


def test_part2_twitter(twitter) -> None:
    assert part2(twitter) == 2223947372407


def test_part1_reddit(reddit) -> None:
    assert part1(reddit) == 991


def test_part2_reddit(reddit) -> None:
    assert part2(reddit) == 1264485568252
