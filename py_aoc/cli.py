from aocd import AOC_TZ
from aocd.get import most_recent_year
import datetime
import argparse


def main():
    aoc_now = datetime.datetime.now(tz=AOC_TZ)
    days = range(1, 26)
    years = range(2015, aoc_now.year + int(aoc_now.month == 12))
    parser = argparse.ArgumentParser(
        description="Advent of Code",
        usage="cli.py [day 1-25] [year 2015-{}]".format(years[-1]),
    )
    parser.add_argument(
        "day",
        nargs="?",
        type=int,
        default=min(aoc_now.day, 25) if aoc_now.month == 12 else 1,
        help="1-25 (default: %(default)s)",
    )
    parser.add_argument(
        "year",
        nargs="?",
        type=int,
        default=most_recent_year(),
        help="2015-{} (default: %(default)s)".format(years[-1]),
    )
    parser.add_argument(
        "--version",
        action="version",
        version="%(prog)s",
    )
    args = parser.parse_args()
    if args.day in years and args.year in days:
        # be forgiving
        args.day, args.year = args.year, args.day
    if args.day not in days or args.year not in years:
        parser.print_usage()
        parser.exit(1)
