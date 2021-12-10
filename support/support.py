from __future__ import annotations

import argparse
import contextlib
import os.path
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Generator, List
from enum import Enum
import json
import importlib

HERE = os.path.dirname(os.path.abspath(__file__))
TOKENS_FILE = os.path.join(".secrets", "tokens.json")


class Tokens(Enum):
    GITHUB = 1
    GOOGLE = 2
    TWITTER = 3
    REDDIT = 4


@contextlib.contextmanager
def timing(name: str = '') -> Generator[None, None, None]:
    before = time.time()
    try:
        yield
    finally:
        after = time.time()
        t = (after - before) * 1000
        unit = 'ms'
        if t < 100:
            t *= 1000
            unit = 'μs'
        if name:
            name = f' ({name})'
        print(f'> {int(t)} {unit}{name}', file=sys.stderr, flush=True)


def _get_cookie_headers(token: Tokens) -> dict[str, str]:
    with open(TOKENS_FILE) as f:
        json_data = json.load(f)
        contents = "session=" + json_data[token.name.lower()]
    return {'Cookie': contents}


def get_input(year: int, day: int, token: Tokens) -> str:
    url = f'https://adventofcode.com/{year}/day/{day}/input'
    req = urllib.request.Request(url, headers=_get_cookie_headers(token))
    return urllib.request.urlopen(req).read().decode()


def get_year_day() -> tuple[int, int]:
    cwd = os.getcwd()
    day_s = os.path.basename(cwd)
    year_s = os.path.basename(os.path.dirname(cwd))

    if not day_s.startswith('day') or not year_s.startswith('aoc'):
        raise AssertionError(f'unexpected working dir: {cwd}')

    return int(year_s[len('aoc'):]), int(day_s[len('day'):])


def download_input() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int)
    parser.add_argument(
        '--token', type=str,
        choices=["github", "google", "twitter", "reddit", "all"],
        default="all",
        required=False
    )
    args = parser.parse_args()
    parser.parse_args()

    day_str = str(args.day).zfill(2)

    tokens_to_download: List[Tokens] = []
    if args.token == "all":
        tokens_to_download.append(Tokens.GITHUB)
        tokens_to_download.append(Tokens.GOOGLE)
        tokens_to_download.append(Tokens.TWITTER)
        tokens_to_download.append(Tokens.REDDIT)
    else:
        tokens_to_download.append(Tokens[str(args.token).upper()])

    for token in tokens_to_download:
        print("processing: " + token.name)

        data_file = os.path.join("data", str(
            args.year), f"day{day_str}_{token.name.lower()}.txt")

        for i in range(5):
            try:
                s = get_input(args.year, args.day, token)
            except urllib.error.URLError as e:
                print(f'zzz: not ready yet: {e}')
                time.sleep(1)
            else:
                break
        else:
            raise SystemExit('timed out after attempting many times')

        with open(data_file, 'w') as f:
            f.write(s)

        lines = s.splitlines()
        if len(lines) > 10:
            for line in lines[: 10]:
                print(line)
            print('...')
        else:
            print(lines[0][: 80])
            print('...')

        time.sleep(1)
    return 0


TOO_QUICK = re.compile('You gave an answer too recently.*to wait.')
WRONG = re.compile(r"That's not the right answer.*?\.")
RIGHT = "That's the right answer!"
ALREADY_DONE = re.compile(r"You don't seem to be solving.*\?")


def submit_solution() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int)
    parser.add_argument('--part', type=int, required=True)
    parser.add_argument(
        '--token', type=str,
        choices=["github", "google", "twitter", "reddit"],
        default="github",
        required=False
    )
    args = parser.parse_args()

    day_str = str(args.day).zfill(2)
    token = Tokens[str(args.token).upper()]
    solution = importlib.import_module(f"py_aoc.year_{args.year}.day{day_str}")
    input_file = os.path.join("data", str(args.year),
                              f"day{day_str}_{token.name.lower()}.txt")
    with open(input_file) as f:
        answer = getattr(solution, f"part{args.part}")(f.read())

    print(f'answer: {answer}')

    params = urllib.parse.urlencode({'level': args.part, 'answer': answer})
    req = urllib.request.Request(
        f'https://adventofcode.com/{args.year}/day/{args.day}/answer',
        method='POST',
        data=params.encode(),
        headers=_get_cookie_headers(token),
    )
    resp = urllib.request.urlopen(req)

    contents = resp.read().decode()

    for error_regex in (WRONG, TOO_QUICK, ALREADY_DONE):
        error_match = error_regex.search(contents)
        if error_match:
            print(f'\033[41m{error_match[0]}\033[m')
            return 1

    if RIGHT in contents:
        print(f'\033[42m{RIGHT}\033[m')
        return 0
    else:
        # unexpected output?
        print(contents)
        return 1
