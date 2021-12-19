import pytest

from py_aoc.year_2021.day18 import INPUT_S, parse, INPUT_GITHUB, INPUT_GOOGLE, INPUT_TWITTER, INPUT_REDDIT, part1, part2, addition, reduce, magnatude
from typing import List, Optional


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
    actual: List[str] = []  # parse(example)
    expected: List[str] = [
    ]
    print(actual)
    assert actual == expected


def test_basic(example: str) -> None:
    input = """\
[1,1]
[2,2]
[3,3]
[4,4]
"""

    def part(ex: str) -> str:
        prev_result: Optional[str] = None
        for i, line in enumerate(ex.splitlines()):
            if i == 0:
                # cannot add just 1 number
                continue
            if prev_result is not None:
                prev_result = addition(prev_result, line)
            else:
                prev_result = addition(ex.splitlines()[i-1], line)
            prev_result = reduce(prev_result)
        if prev_result is None:
            return ""
        else:
            return prev_result
    assert part(input) == "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    input = """\
[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
"""
    print("-----")
    ans = part(input)
    print(".....:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    print(f'ans  : "{ans}"')
    assert ans == "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"

    print("magnatude")
    assert magnatude(ans) == 1384


def test_part1_example(example: str) -> None:
    assert(part1(example) == 4140)


def test_part2_example(example: str) -> None:
    assert(part2(example) == 3993)


def test_part1_github(github: str) -> None:
    assert part1(github) == 3359


def test_part2_github(github: str) -> None:
    assert part2(github) == 4616


def test_part1_google(google: str) -> None:
    assert part1(google) == 3816


def test_part2_google(google: str) -> None:
    assert part2(google) == 4819


def test_part1_twitter(twitter: str) -> None:
    assert part1(twitter) == 3806


def test_part2_twitter(twitter: str) -> None:
    assert part2(twitter) == 4727


def test_part1_reddit(reddit: str) -> None:
    assert part1(reddit) == 3884


def test_part2_reddit(reddit: str) -> None:
    assert part2(reddit) == 4595
