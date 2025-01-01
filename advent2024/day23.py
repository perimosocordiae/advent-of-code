import networkx as nx


def main(infile="inputs/23.full"):
    g = nx.Graph()
    nodes = set()
    for line in open(infile):
        lhs, rhs = line.strip().split("-")
        g.add_edge(lhs, rhs)
        if lhs.startswith("t"):
            nodes.add(lhs)
        if rhs.startswith("t"):
            nodes.add(rhs)

    # Part 2: find the largest clique
    clique, _ = nx.max_weight_clique(g, weight=None)

    # Part 1: count unique triangles including a entry in `nodes`.
    count = 0
    for n in nodes:
        count += nx.triangles(g, n)
        g.remove_node(n)
    print("Part 1:", count)
    print("Part 2:", ",".join(sorted(clique)))


if __name__ == "__main__":
    main()
