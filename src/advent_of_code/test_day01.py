import pytest
from src.advent_of_code.day01 import compute

INPUT_S = """\
199
200
208
210
200
207
240
269
260
263
"""


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, 5),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s) == expected
