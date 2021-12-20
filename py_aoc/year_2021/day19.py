from __future__ import annotations

import argparse
import os.path
import math
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray
from enum import Enum

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day19_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day19_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day19_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day19_twitter.txt")
INPUT_S = """\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"""

Point_3D = Tuple[int, int, int]


class Transform(Enum):
    T1 = 1
    T2 = 2
    T3 = 3
    T4 = 4
    T5 = 5
    T6 = 6
    T7 = 7
    T8 = 8
    T9 = 9
    T10 = 10
    T11 = 11
    T12 = 12
    T13 = 13
    T14 = 14
    T15 = 15
    T16 = 16
    T17 = 17
    T18 = 18
    T19 = 19
    T20 = 20
    T21 = 21
    T22 = 22
    T23 = 23
    T24 = 24


class Scanner(NamedTuple):
    id: int
    points: List[Point_3D]


class Axis(NamedTuple):
    sign: int
    axis: int
    diff: int


def parse(input: str) -> Dict[int, Scanner]:
    scanners: Dict[int, Scanner] = {}
    lines = input.splitlines()
    points: List[Point_3D]
    cur_id = 0
    for line in lines:
        if line.startswith("---"):
            _, _, id, _ = line.split()
            scanners[int(id)] = Scanner(int(id), [])
            cur_id = int(id)
            continue
        elif line == "":
            continue
        else:
            x, y, z = line.split(',')
            scanners[cur_id].points.append((int(x), int(y), int(z)))

    return scanners


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


def check_overlap(scanner0: Scanner, scanner1: Scanner) -> Tuple[List[Point_3D], List[Point_3D]]:
    distance0: Dict[Tuple[Point_3D, Point_3D], float] = {}
    for point1 in scanner0.points:
        for point2 in scanner0.points:
            if point1 == point2:
                continue
            # check reverse
            if (point2, point1) in distance0:
                continue
            distance0[(point1, point2)] = math.sqrt(
                (point1[0] - point2[0]) ** 2 +
                (point1[1] - point2[1]) ** 2 +
                (point1[2] - point2[2]) ** 2
            )
    distance1: Dict[Tuple[Point_3D, Point_3D], float] = {}
    for point1 in scanner1.points:
        for point2 in scanner1.points:
            if point1 == point2:
                continue
            if (point2, point1) in distance1:
                continue
            distance1[(point1, point2)] = math.sqrt(
                (point1[0] - point2[0]) ** 2 +
                (point1[1] - point2[1]) ** 2 +
                (point1[2] - point2[2]) ** 2
            )
    matched_beacons_0: List[Point_3D] = []
    matched_beacons_1: List[Point_3D] = []
    for pointa, dist1 in distance0.items():
        for pointb, dist2 in distance1.items():
            if dist1 == 0 or dist2 == 0:
                continue
            if dist1 == dist2:
                if pointa[0] not in matched_beacons_0:
                    matched_beacons_0.append(pointa[0])
                if pointa[1] not in matched_beacons_0:
                    matched_beacons_0.append(pointa[1])
                if pointb[0] not in matched_beacons_1:
                    matched_beacons_1.append(pointb[0])
                if pointb[1] not in matched_beacons_1:
                    matched_beacons_1.append(pointb[1])
                break
    return matched_beacons_0, matched_beacons_1


def get_transform(vec0: Tuple[int, int, int], vec1: Tuple[int, int, int]) -> Transform:
    x0 = vec0[0]
    y0 = vec0[1]
    z0 = vec0[2]
    x1 = vec1[0]
    y1 = vec1[1]
    z1 = vec1[2]

    if abs(x0) == abs(x1) and abs(y0) == abs(y1) and abs(z0) == abs(z1):
        if x0 == -x1 and y0 == y1 and z0 == -z1:
            return Transform.T1
    else:
        if x0 == y1 and y0 == z1 and z0 == -x1:
            return Transform.T2
        elif x0 == y1 and y0 == -z1 and z0 == -x1:
            return Transform.T3

    print(vec0)
    print(vec1)

    raise AssertionError("")


def perform_transform(point: Point_3D, t: Transform) -> Point_3D:
    if t == Transform.T1:
        return (point[0]*-1, point[1], point[2] * -1)
    elif t == Transform.T2:
        return (point[1], point[2], point[0] * -1)
    elif t == Transform.T3:
        return (point[1], point[2] * -1, point[0] * -1)

    raise AssertionError("")


"""
TODO
4) Apply the set of rotation matrices to each point in scanner 1's and compare
the result to the matching point in scanner 0's set. If all the results are THE
SAME for each pair, BINGO you have found the correct rotation matrix. And
probably also the scanner 1 point of reference... ;-)

5) Apply the found rotation matrix and point of reference to ALL the beacons
found in scanner 1, and add the newly calculated beacon references (which will
be in the scanner 0 frame of reference) to the list of known good beacon
references (also in the scanner 0 frame of reference).
"""


def part1_orig(input: str) -> int:
    scanners: Dict[int, Scanner] = parse(input)

    reference_scanner = scanners[0]

    scanner_to_try = scanners[1]
    list_ref, list_try = check_overlap(
        reference_scanner, scanner_to_try)

    if len(list_ref) >= 12 and len(list_try) >= 12:
        print(f"scanner {scanner_to_try.id} overlaps with reference")
        print(list_ref)
        print(list_try)
        vec0 = (
            list_ref[1][0] - list_ref[0][0],
            list_ref[1][1] - list_ref[0][1],
            list_ref[1][2] - list_ref[0][2]
        )
        print(vec0)
        vec1 = (
            list_try[1][0] - list_try[0][0],
            list_try[1][1] - list_try[0][1],
            list_try[1][2] - list_try[0][2]
        )
        print(vec1)
        transform = get_transform(vec0, vec1)
        tsf = perform_transform(list_try[0], transform)
        print(list_ref[0])
        print(list_try[0])
        print(tsf)
        scanner_diff = (
            list_ref[0][0] - tsf[0],
            list_ref[0][1] - tsf[1],
            list_ref[0][2] - tsf[2]
            # tsf[0] - list_ref[0][0],
            # tsf[1] - list_ref[0][1],
            # tsf[2] - list_ref[0][2]
        )

        for point in scanner_to_try.points:
            new_pt = (
                point[0] + scanner_diff[0],
                point[1] + scanner_diff[1],
                point[2] + scanner_diff[2]
            )
            if new_pt not in reference_scanner.points:
                reference_scanner.points.append(new_pt)

        print(f"scanner diff is {scanner_diff}")
    else:
        print(f"scanner {scanner_to_try.id} does not map")

    scanner_to_try = scanners[4]
    list_ref, list_try = check_overlap(
        scanners[1], scanner_to_try)

    if len(list_ref) >= 12 and len(list_try) >= 12:
        print(f"scanner {scanner_to_try.id} overlaps with reference")
        print(list_ref)
        print(list_try)
        vec0 = (
            list_ref[1][0] - list_ref[0][0],
            list_ref[1][1] - list_ref[0][1],
            list_ref[1][2] - list_ref[0][2]
        )
        print(f"vec0: {vec0}")
        vec1 = (
            list_try[1][0] - list_try[0][0],
            list_try[1][1] - list_try[0][1],
            list_try[1][2] - list_try[0][2]
        )
        print(f"vec1: {vec1}")
        transform = get_transform(vec0, vec1)
        tsf = perform_transform(list_try[0], transform)
        print("")
        print(list_ref[0])
        print(list_try[0])
        print(tsf)
        scanner_diff = (
            list_ref[0][0] - tsf[0],
            list_ref[0][1] - tsf[1],
            list_ref[0][2] - tsf[2]
        )
        print(f"scanner diff is {scanner_diff}")
        scanner_diff1 = (
            68 - scanner_diff[0],
            -1246 - scanner_diff[1],
            -43 - scanner_diff[2]
        )
        print(f"scanner diff to 0 is {scanner_diff1}")
    else:
        print(f"scanner {scanner_to_try.id} does not map")

    return 0


def find_if_overlap(ref: Scanner, others: Dict[int, Scanner]) -> Dict[int, Axis]:
    x_axis: Dict[int, Axis] = {}

    for other in others.values():
        for axis in (0, 1, 2):
            for sign in (-1, 1):
                dx: collections.Counter[int] = collections.Counter()
                for pt in ref.points:
                    for otherpt in other.points:
                        dx[pt[0] - otherpt[axis] * sign] += 1
                mc = dx.most_common(1)[0]
                if mc[1] >= 12:
                    x_axis[other.id] = Axis(
                        axis=axis,
                        sign=sign,
                        diff=mc[0]
                    )
    return x_axis


def find_other_axis(ref: Scanner, x_axises: Dict[int, Axis], others: Dict[int, Scanner]) -> Tuple[Dict[int, Axis], Dict[int, Axis]]:
    y_axis: Dict[int, Axis] = {}
    z_axis: Dict[int, Axis] = {}

    for id in x_axises:
        other = others[id]
        for axis in (0, 1, 2):
            for sign in (-1, 1):
                dy: collections.Counter[int] = collections.Counter()
                dz: collections.Counter[int] = collections.Counter()
                for pt in ref.points:
                    for otherpt in other.points:
                        dy[pt[1] - otherpt[axis] * sign] += 1
                        dz[pt[2] - otherpt[axis] * sign] += 1

                mc = dy.most_common(1)[0]
                if mc[1] >= 12:
                    y_axis[other.id] = Axis(
                        axis=axis,
                        sign=sign,
                        diff=mc[0]
                    )
                mc = dz.most_common(1)[0]
                if mc[1] >= 12:
                    z_axis[other.id] = Axis(
                        axis=axis,
                        sign=sign,
                        diff=mc[0]
                    )
    return y_axis, z_axis


def part1(input: str) -> int:
    scanners: Dict[int, Scanner] = parse(input)

    ref = scanners.pop(0)
    relative_scanner_locations = {0: (0, 0, 0)}
    beacons: Set[Point_3D] = set(ref.points)

    # x_axis = find_if_overlap(ref, {1: scanners[1]})
    # y_axis, z_axis = find_other_axis(ref, x_axis, scanners)
    # print(x_axis)
    # print(y_axis)
    # print(z_axis)

    # x_axis = find_if_overlap(ref1, {1: scanners[4]})
    # y_axis, z_axis = find_other_axis(ref1, x_axis, scanners)
    # print(x_axis)
    # print(y_axis)
    # print(z_axis)
    to_process = [ref]
    while to_process:
        src = to_process.pop()
        x_axis = find_if_overlap(src, scanners)
        y_axis, z_axis = find_other_axis(src, x_axis, scanners)

        for id in x_axis:
            relative_scanner_locations[id] = (
                x_axis[id].diff, y_axis[id].diff, z_axis[id].diff)
            next = scanners.pop(id)
            next.points[:] = [
                (
                    x_axis[id].diff + x_axis[id].sign * pt[x_axis[id].axis],
                    y_axis[id].diff + y_axis[id].sign * pt[y_axis[id].axis],
                    z_axis[id].diff + z_axis[id].sign * pt[z_axis[id].axis],
                )
                for pt in next.points
            ]
            beacons.update(next.points)
            to_process.append(next)
    print(relative_scanner_locations)

    return len(beacons)


def part2(input: str) -> int:
    scanners: Dict[int, Scanner] = parse(input)

    ref = scanners.pop(0)
    relative_scanner_locations = {0: (0, 0, 0)}
    beacons: Set[Point_3D] = set(ref.points)

    # x_axis = find_if_overlap(ref, {1: scanners[1]})
    # y_axis, z_axis = find_other_axis(ref, x_axis, scanners)
    # print(x_axis)
    # print(y_axis)
    # print(z_axis)

    # x_axis = find_if_overlap(ref1, {1: scanners[4]})
    # y_axis, z_axis = find_other_axis(ref1, x_axis, scanners)
    # print(x_axis)
    # print(y_axis)
    # print(z_axis)
    to_process = [ref]
    while to_process:
        src = to_process.pop()
        x_axis = find_if_overlap(src, scanners)
        y_axis, z_axis = find_other_axis(src, x_axis, scanners)

        for id in x_axis:
            relative_scanner_locations[id] = (
                x_axis[id].diff, y_axis[id].diff, z_axis[id].diff)
            next = scanners.pop(id)
            next.points[:] = [
                (
                    x_axis[id].diff + x_axis[id].sign * pt[x_axis[id].axis],
                    y_axis[id].diff + y_axis[id].sign * pt[y_axis[id].axis],
                    z_axis[id].diff + z_axis[id].sign * pt[z_axis[id].axis],
                )
                for pt in next.points
            ]
            beacons.update(next.points)
            to_process.append(next)

    max_dist = 0
    positions = list(relative_scanner_locations.values())
    for i, (x1, y1, z1) in enumerate(positions):
        for x2, y2, z2 in positions[i:]:
            max_dist = max(
                abs(x2 - x1) + abs(y2 - y1) + abs(z2 - z1),
                max_dist,
            )

    return max_dist


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_GITHUB)
    args = parser.parse_args()

    # with open(args.data_file) as f, timing():
    solutions = solve(INPUT_S)
    print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
