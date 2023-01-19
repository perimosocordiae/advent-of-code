#!/usr/bin/env python3
import math
from dataclasses import dataclass


@dataclass
class Packet:
    version: int
    type_id: int
    subpackets: list["Packet"]
    value: int = -1

    def version_sum(self) -> int:
        return self.version + sum(s.version_sum() for s in self.subpackets)

    def evaluate(self) -> int:
        if self.type_id == 4:
            assert self.value != -1
            return self.value
        if self.type_id == 0:
            return sum(s.evaluate() for s in self.subpackets)
        if self.type_id == 1:
            return math.prod(s.evaluate() for s in self.subpackets)
        if self.type_id == 2:
            return min(s.evaluate() for s in self.subpackets)
        if self.type_id == 3:
            return max(s.evaluate() for s in self.subpackets)
        # Comparison operators
        assert len(self.subpackets) == 2
        lhs = self.subpackets[0].evaluate()
        rhs = self.subpackets[1].evaluate()
        if self.type_id == 5:
            return int(lhs > rhs)
        if self.type_id == 6:
            return int(lhs < rhs)
        if self.type_id == 7:
            return int(lhs == rhs)
        raise ValueError(f"Unknown type_id {self.type_id}")


def parse_packet(bits: str) -> tuple[Packet, int]:
    packet = Packet(version=int(bits[:3], 2), type_id=int(bits[3:6], 2), subpackets=[])
    if packet.type_id == 4:
        # literal value
        i = 6
        val_bits = ""
        while bits[i] == "1":
            val_bits += bits[i + 1 : i + 5]
            i += 5
        val_bits += bits[i + 1 : i + 5]
        packet.value = int(val_bits, 2)
        return packet, i + 5
    # operator packets
    if bits[6] == "0":
        i = 7 + 15
        stop_idx = i + int(bits[7:i], 2)
        while i < stop_idx:
            subpacket, j = parse_packet(bits[i:])
            packet.subpackets.append(subpacket)
            i += j
    else:
        i = 7 + 11
        num_subpackets = int(bits[7:i], 2)
        for _ in range(num_subpackets):
            subpacket, j = parse_packet(bits[i:])
            packet.subpackets.append(subpacket)
            i += j
    return packet, i


def decode_transmission(data: str) -> Packet:
    bits = bin(int(data, 16))[2:]
    bits = bits.rjust(len(data) * 4, "0")
    p, _ = parse_packet(bits)
    return p


def solve(path: str) -> None:
    data = open(path).read().strip()
    packet = decode_transmission(data)
    print("Part 1:", packet.version_sum())
    print("Part 2:", packet.evaluate())


if __name__ == "__main__":
    solve("inputs/16.full")
