import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class day08 {
  public static void main(String[] args) {
    String filePath = "inputs/08.full";

    // Store the graph: node id -> (left node, right node)
    HashMap<String, Map.Entry<String, String>> graph = new HashMap<>();
    String instructions = "";

    try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
      instructions = br.readLine().trim();
      br.readLine(); // Skip the blank line

      String line;
      while ((line = br.readLine()) != null) {
        // Line format: "AAA = (BBB, CCC)"
        String[] parts = line.trim().split(" = ");
        String nodeId = parts[0];
        String[] children = parts[1].substring(1, parts[1].length() - 1).split(", ");
        graph.put(nodeId, Map.entry(children[0], children[1]));
      }
    } catch (IOException e) {
      e.printStackTrace();
    }

    System.out.println("Part 1: " + part1(graph, instructions));
    System.out.println("Part 2: " + part2(graph, instructions));
  }

  public static int countSteps(
      HashMap<String, Map.Entry<String, String>> graph,
      String instructions, String startNode, Set<String> endNodes) {
    int numSteps = 0;
    String nodeId = startNode;

    while (!endNodes.contains(nodeId)) {
      Map.Entry<String, String> children = graph.get(nodeId);
      char inst = instructions.charAt(numSteps % instructions.length());
      switch (inst) {
        case 'L':
          nodeId = children.getKey();
          break;
        case 'R':
          nodeId = children.getValue();
          break;
        default:
          throw new IllegalArgumentException("Invalid instruction: " + inst);
      }
      numSteps++;
    }

    return numSteps;
  }

  public static int part1(
      HashMap<String, Map.Entry<String, String>> graph,
      String instructions) {
    return countSteps(graph, instructions, "AAA", Set.of("ZZZ"));
  }

  public static long part2(HashMap<String, Map.Entry<String, String>> graph, String instructions) {
    // Find all nodes that end with 'Z'.
    Set<String> endNodes = graph.keySet().stream().filter(nodeId -> nodeId.endsWith("Z"))
        .collect(java.util.stream.Collectors.toSet());
    // Solve for each start node, then combine their unique prime factors.
    return graph.keySet().stream().filter(nodeId -> nodeId.endsWith("A"))
        .map(nodeId -> primeFactors(countSteps(graph, instructions, nodeId, endNodes)))
        .reduce(null, (a, b) -> {
          if (a == null) {
            return b;
          }
          a.addAll(b);
          return a;
        }).stream().reduce(1L, (a, b) -> a * b);
  }

  public static Set<Long> primeFactors(int n) {
    Set<Long> factors = new java.util.HashSet<>();
    while (n % 2 == 0) {
      factors.add(2L);
      n /= 2;
    }
    for (int i = 3; i <= Math.sqrt(n); i += 2) {
      while (n % i == 0) {
        factors.add((long) i);
        n /= i;
      }
    }
    if (n > 2) {
      factors.add((long) n);
    }
    return factors;
  }
}
