from aocd import AOC_TZ
from aocd.get import most_recent_year, get_data
from aocd.runner import _load_users
import datetime
import argparse
import os
import time
from os.path import exists


def main():

    def now() -> datetime:
        return datetime.datetime.now(tz=AOC_TZ)

    aoc_now = now()
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

    args = parser.parse_args()
    if args.day in years and args.year in days:
        # be forgiving
        args.day, args.year = args.year, args.day
    if args.day not in days or args.year not in years:
        parser.print_usage()
        parser.exit(1)

    day_str = str(args.day).zfill(2)

    # Create new python file and pytest file
    template_filename = os.path.join(
        os.path.dirname(__file__), "template.py")
    python_filename = os.path.join(
        os.path.dirname(__file__), f"day{day_str}.py")
    test_template_filename = os.path.join(
        os.path.dirname(__file__), "template_pytest.py")
    python_test_filename = os.path.join(
        os.path.dirname(__file__), f"test_day{day_str}.py")

    day_file_exists = exists(python_filename)
    if not day_file_exists:
        print("writing templates...")
        with open(python_filename, "w") as f, open(template_filename) as template:
            f.write(template.read())
        with open(python_test_filename, "w") as f, open(test_template_filename) as template:
            f.write(template.read())

    # Download input txt file from AoC website
    github_data = os.path.join(os.path.dirname(
        __file__), "aoc_data", f"day{day_str}_github.txt")
    twitter_data = os.path.join(os.path.dirname(
        __file__), "aoc_data", f"day{day_str}_twitter.txt")
    reddit_data = os.path.join(os.path.dirname(
        __file__), "aoc_data", f"day{day_str}_reddit.txt")
    google_data = os.path.join(os.path.dirname(
        __file__), "aoc_data", f"day{day_str}_google.txt")

    users = _load_users()
    for user, token in users.items():
        print(user)
        if user == "github" and not exists(github_data):
            print("writing github_data...")
            with open(github_data, "w") as f:
                f.write(get_data(session=token, day=args.day, year=args.year))
        elif user == "google" and not exists(google_data):
            print("writing google_data...")
            with open(google_data, "w") as f:
                f.write(get_data(session=token, day=args.day, year=args.year))
        elif user == "twitter" and not exists(twitter_data):
            print("writing twitter_data...")
            with open(twitter_data, "w") as f:
                f.write(get_data(session=token, day=args.day, year=args.year))
        elif user == "reddit" and not exists(reddit_data):
            print("writing reddit_data...")
            with open(reddit_data, "w") as f:
                f.write(get_data(session=token, day=args.day, year=args.year))
