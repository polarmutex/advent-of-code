import pytest
from src.advent_of_code.day03 import compute

INPUT_S = """\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 230),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s) == expected
