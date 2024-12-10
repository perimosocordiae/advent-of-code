import networkx as nx


def parse_graph(filepath: str):
    numbers = [[int(x) for x in line.strip()] for line in open(filepath)]
    g = nx.DiGraph()
    sources = []
    sinks = []
    for i, row in enumerate(numbers):
        for j, val in enumerate(row):
            g.add_node((i, j))
            if val == 0:
                sources.append((i, j))
            elif val == 9:
                sinks.append((i, j))
                continue
            next_val = val + 1
            if i > 0 and numbers[i - 1][j] == next_val:
                g.add_edge((i, j), (i - 1, j))
            if j > 0 and numbers[i][j - 1] == next_val:
                g.add_edge((i, j), (i, j - 1))
            if i < len(numbers) - 1 and numbers[i + 1][j] == next_val:
                g.add_edge((i, j), (i + 1, j))
            if j < len(row) - 1 and numbers[i][j + 1] == next_val:
                g.add_edge((i, j), (i, j + 1))
    return g, sources, sinks


# Adapted from networkx's _build_paths_from_predecessors function.
def count_paths(src, sink, pred) -> int:
    if sink not in pred:
        return 0
    seen = {sink}
    stack = [[sink, 0]]
    top = 0
    num_paths = 0
    while top >= 0:
        node, i = stack[top]
        if node == src:
            num_paths += 1
        if len(pred[node]) > i:
            stack[top][1] = i + 1
            next_node = pred[node][i]
            if next_node in seen:
                continue
            seen.add(next_node)
            top += 1
            if top == len(stack):
                stack.append([next_node, 0])
            else:
                stack[top][:] = [next_node, 0]
        else:
            seen.discard(node)
            top -= 1
    return num_paths


infile = "inputs/10.full"
g, sources, sinks = parse_graph(infile)
part1 = 0
part2 = 0
for src in sources:
    pred = nx.predecessor(g, src)
    for sink in sinks:
        if num_paths := count_paths(src, sink, pred):
            part1 += 1
            part2 += num_paths
print("Part 1:", part1)
print("Part 2:", part2)
