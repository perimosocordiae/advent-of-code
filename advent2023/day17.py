import collections
import heapq
import numpy as np

class Node:
  def __init__(self, pos, prev_dir, prev_dir_count):
    self.pos = pos
    self.prev_dir = prev_dir
    self.prev_dir_count = prev_dir_count

  def __hash__(self) -> int:
    return hash((self.pos[0], self.pos[1], self.prev_dir, self.prev_dir_count))

  def __eq__(self, o: object) -> bool:
    return (self.pos[0] == o.pos[0] and
            self.pos[1] == o.pos[1] and
            self.prev_dir == o.prev_dir and
            self.prev_dir_count == o.prev_dir_count)

  def __repr__(self) -> str:
    return f'Node({self.pos}, {self.prev_dir}, {self.prev_dir_count})'

  def __lt__(self, o: object) -> bool:
    return self.pos.sum() < o.pos.sum()

# TODO: Represent the full state in the graph:
# - current position
# - last 3 directions
def minimize_heat_loss(grid):
  goal = np.array(grid.shape, dtype=int) - 1

  # down, right, up, left
  DIRECTIONS = np.array([[1, 0], [0, 1], [-1, 0], [0, -1]], dtype=int)

  queue = [(0, Node(np.array([0, 0]), 0, 0))]
  heapq.heapify(queue)
  cost = collections.defaultdict(lambda: 99999)
  best_final_cost = 99999
  while queue:
    curr_cost, node = heapq.heappop(queue)
    if cost[node] <= curr_cost or curr_cost >= best_final_cost:
      continue
    cost[node] = curr_cost
    # print(node, curr_cost)
    for d, delta in enumerate(DIRECTIONS):
      if d == node.prev_dir:
        if node.prev_dir_count >= 2:
          continue
        new_count = node.prev_dir_count + 1
      elif abs(d - node.prev_dir) == 2:
        continue
      else:
        new_count = 0
      new_pos = node.pos + delta
      if (new_pos < 0).any() or (new_pos > goal).any():
        continue
      new_cost = curr_cost + grid[tuple(new_pos)]
      if new_cost >= best_final_cost:
        continue
      new_node = Node(new_pos, d, new_count)
      if new_cost < cost[new_node]:
        if (new_pos == goal).all():
          print(new_cost)
          best_final_cost = min(new_cost, best_final_cost)
        else:
          heapq.heappush(queue, (new_cost, new_node))
  return best_final_cost


grid = np.array([[int(c) for c in line.strip()]
                 for line in open('inputs/17.full')], dtype=int)
print(grid.shape)
print('Part 1:', minimize_heat_loss(grid))
