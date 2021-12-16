from __future__ import annotations

import argparse
import os.path
import collections
from collections import Counter
from typing import List, Tuple, Generator, Set, Dict, NamedTuple
from bitarray.util import hex2ba, ba2int, int2ba
from bitarray import bitarray

from support import timing

INPUT_GITHUB = os.path.join("data", "2021", "day16_github.txt")
INPUT_GOOGLE = os.path.join("data", "2021", "day16_google.txt")
INPUT_REDDIT = os.path.join("data", "2021", "day16_reddit.txt")
INPUT_TWITTER = os.path.join("data", "2021", "day16_twitter.txt")
INPUT_S = """\
D2FE28
38006F45291200
EE00D40C823060
8A004A801A8002F478
620080001611562C8802118E34
C0015000016115A2E0802F182340
A0016C880162017C3686B18A3D4780
"""
INPUT_S2 = """\
C200B40A82
04005AC33890
880086C3E88112
CE00C43D881120
D8005AC2A8F0
F600BC2D8F
9C005AC2F8F0
9C0141080250320F1802104A08
"""

PolymerTemplate = str
Point = Tuple[int, int]


def parse(input: str) -> []:
    map = []
    for index, line in enumerate(input.splitlines()):
        map.append(line)
    return map


def solve(puzzle_input: str) -> Tuple[int, int]:
    solution1 = part1(puzzle_input)
    solution2 = part2(puzzle_input)
    return solution1, solution2


class Packet(NamedTuple):
    version: int
    id: int
    val: int = -1
    packets: List[Packet] = []


class BITS:
    def __init__(self, input: str):
        print(f"initial message: {input}")
        valid = [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"
        ]
        for i, char in enumerate(input):
            if char not in valid:
                print(f" '{char}' ")
                print(i)
        self.message = hex2ba(input.rstrip())
        self.index = 0
        print(f"initial message: {self.message}")

    def read(self, num_bits: int) -> int:
        if num_bits == 1:
            val = self.message[self.index]
        else:
            val = ba2int(self.message[self.index:self.index + num_bits])
        self.index += num_bits
        return val

    def parse_packet(self, i: int) -> Tuple[int, Packet]:
        version = self.read(3)
        id = self.read(3)
        print(f"pkt hdr - version: {version} id: {id}")

        if id == 4:
            # Literal Value
            done = True
            literal_ba: bitarray = bitarray()
            while done:
                done = self.read(1) == 1
                print(f"\tcontinue: {done}")
                value = self.read(4)
                literal_ba.extend(int2ba(value, length=4))
            value = ba2int(literal_ba)
            print(f"\tliteral value: {value}")
            # forward extra bits
            return self.index - i, Packet(version=version, id=id, val=value)
        else:
            # Operator
            lengh_id_type = self.read(1)
            print(f"\tlength id type: {lengh_id_type}")
            packets: List[Packet] = []
            if lengh_id_type == 0:
                bit_length: int = self.read(15)
                print(f"\tlength: {bit_length}")
                while bit_length > 5:
                    print(bit_length)
                    num_read, packet = self.parse_packet(self.index)
                    packets.append(packet)
                    bit_length -= num_read
                print("done with operator (bits)")
                return self.index - i, Packet(version=version, id=id, packets=packets)
            else:
                num_packets: int = self.read(11)
                print(f"\tnum packets: {num_packets}")
                for _ in range(num_packets):
                    _, packet = self.parse_packet(self.index)
                    packets.append(packet)
                return self.index - i, Packet(version=version, id=id, packets=packets)
        return self.index - i, Packet(version=version, id=id, packets=packets)

    def parse(self) -> Packet:
        self.index = 0
        i, packet = self.parse_packet(self.index)
        return packet

    def parse_packet_header(self, header: bitarray) -> Tuple[int, int]:
        assert(len(header) == 6)
        version = ba2int(header[0:3])
        id = ba2int(header[3:6])
        return version, id

    def parse_literal_packet(self, bits: bitarray) -> Tuple[bool, int]:
        assert(len(bits) == 5)
        kepp_parsing = bits[0] == 1
        value = ba2int(bits[1:5])
        return kepp_parsing, value


def part1(input: str) -> int:
    bits = BITS(input)
    packet = bits.parse()

    total_versions: int = 0
    process = []
    process.append(packet)
    while process:
        pkt = process.pop()
        total_versions += pkt.version
        process.extend(pkt.packets)

    return total_versions


def part2(input: str) -> int:
    bits = BITS(input)
    packet = bits.parse()

    def compute(packet: Packet) -> int:
        if packet.id == 0:
            temp = 0
            for pkt in packet.packets:
                temp += compute(pkt)
            return temp
        elif packet.id == 1:
            temp = 1
            for pkt in packet.packets:
                temp *= compute(pkt)
            return temp
        elif packet.id == 2:
            return min(compute(pkt) for pkt in packet.packets)
        elif packet.id == 3:
            return max(compute(pkt) for pkt in packet.packets)
        elif packet.id == 4:
            return packet.val
        elif packet.id == 5:
            return 1 if compute(packet.packets[0]) > compute(packet.packets[1]) else 0
        elif packet.id == 6:
            return 1 if compute(packet.packets[0]) < compute(packet.packets[1]) else 0
        elif packet.id == 7:
            return 1 if compute(packet.packets[0]) == compute(packet.packets[1]) else 0
        raise AssertionError(packet)
    return compute(packet)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("data_file", nargs="?", default=INPUT_GITHUB)
    args = parser.parse_args()

    with open(args.data_file) as f, timing():
        solutions = solve(f.read())
        print("\n".join(str(solution) for solution in solutions))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
