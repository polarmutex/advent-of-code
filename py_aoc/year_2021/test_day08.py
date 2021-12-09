import pytest

from py_aoc.year_2021.day08 import INPUT_S, parse, INPUT_TXT, part1, part2


@pytest.fixture
def example():
    return INPUT_S


@pytest.fixture
def input():
    with open(INPUT_TXT) as f:
        return f.read()


def test_parse(example):
    actual = parse(example)
    expected = [
        (["be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd", "edb"], [
         "fdgacbe", "cefdb", "cefbgd", "gcbe"]),
        (["edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd",
         "abcde", "gfcbed", "gfec", ], ["fcgedb", "cgb", "dgebacf", "gc", ]),
        (["fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad",
         "gfac", "gcb", "cdgabef", ], ["cg", "cg", "fdcagb", "cbg", ]),
        (["fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab",
         "fgdeca", "fcdbega", ], ["efabcd", "cedba", "gadfec", "cb", ]),
        (["aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb",
         "dgceab", "fcbdga", ], ["gecf", "egdcabf", "bgf", "bfgea", ]),
        (["fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec",
         "bfadeg", "bafgc", "acf", ], ["gebdcfa", "ecba", "ca", "fadegcb", ]),
        (["dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc",
         "dacgb", "gdcebf", "gf", ], ["cefg", "dcbef", "fcge", "gbcadfe", ]),
        (["bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf",
         "ced", "adcbefg", "gebcd", ], ["ed", "bcgafe", "cdgba", "cbgef", ]),
        (["egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg",
         "fgcdab", "egfdb", "bfceg", ], ["gbdfcae", "bgc", "cg", "cgb", ]),
        (["gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef",
         "cafbge", "fdbac", "fegbdc", ], ["fgae", "cfgab", "fg", "bagce", ]),
    ]
    assert actual == expected


def test_part1_example(example) -> None:
    assert part1(example) == 26


def test_part2_example(example) -> None:
    assert part2(example) == 61229


@pytest.mark.skip(reason="Not implemented")
def test_part1_input(input) -> None:
    assert part1(input) == 388739


@pytest.mark.skip(reason="Not implemented")
def test_part2_input(input) -> None:
    assert part2(input) == 1741362314973
