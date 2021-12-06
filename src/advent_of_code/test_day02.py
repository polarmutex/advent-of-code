import pytest
from src.advent_of_code.day02 import compute

INPUT_S = """\
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 900),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s) == expected
