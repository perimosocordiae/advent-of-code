import numpy as np
import matplotlib.pyplot as plt
from scipy.ndimage import label


def _parse_pair(s: str) -> tuple[int, int]:
    return tuple(map(int, s[2:].split(",")))


def _parse_file(path: str) -> tuple[np.ndarray, np.ndarray]:
    positions = []
    velocities = []
    for line in open(path):
        pos, vel = map(_parse_pair, line.strip().split())
        positions.append(pos)
        velocities.append(vel)
    return np.array(positions), np.array(velocities)


positions, velocities = _parse_file("inputs/14.full")
room_size = (101, 103)


def part1(positions, velocities, room_size) -> int:
    final = positions + 100 * velocities
    final = np.remainder(final, room_size)

    mid = room_size[0] // 2, room_size[1] // 2
    top = final[:, 0] < mid[0]
    bottom = final[:, 0] > mid[0]
    left = final[:, 1] < mid[1]
    right = final[:, 1] > mid[1]

    quads = (
        np.count_nonzero(top & left),
        np.count_nonzero(top & right),
        np.count_nonzero(bottom & left),
        np.count_nonzero(bottom & right),
    )
    return int(np.prod(quads))


def _display(positions, room_size, title: str) -> None:
    room = np.zeros(room_size, dtype=int)
    room[*positions.T] = 1
    _, num_objs = label(room)

    if num_objs < 300:
        plt.imshow(room, interpolation="nearest")
        plt.title(title)
        plt.show()


print("Part 1:", part1(positions, velocities, room_size))
for i in range(1000):
    positions += velocities
    positions = np.remainder(positions, room_size)
    _display(positions, room_size, title=f"Seconds: {i+1}")
