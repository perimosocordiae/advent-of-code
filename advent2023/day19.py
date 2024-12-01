from math import prod


class GreaterThan:
  def __init__(self, idx: int, val: int):
    self.idx = idx
    self.val = val

  def __call__(self, x):
    return x[self.idx] > self.val

  def __repr__(self) -> str:
    ch = 'xmas'[self.idx]
    return f'({ch} > {self.val})'


class LessThan:
  def __init__(self, idx: int, val: int):
    self.idx = idx
    self.val = val

  def __call__(self, x):
    return x[self.idx] < self.val

  def __repr__(self) -> str:
    ch = 'xmas'[self.idx]
    return f'({ch} < {self.val})'

def accept(x):
  return True

def parse_rule(rule: str):
  if ':' not in rule:
    return rule, accept
  cond, succ = rule.split(':')
  idx = 'xmas'.index(cond[0])
  op = cond[1]
  val = int(cond[2:])
  if op == '>':
    fn = GreaterThan(idx, val)
  elif op == '<':
    fn = LessThan(idx, val)
  else:
    raise ValueError(f'Invalid op in {rule}')
  return succ, fn


def apply_workflow(workflow, rating):
  for succ, fn in workflow:
    if fn(rating):
      return succ
  raise ValueError(f'No rule matched {rating}')


def process_rating(workflows, rating):
  succ = 'in'
  while True:
    succ = apply_workflow(workflows[succ], rating)
    if succ == 'A':
      return sum(rating)
    if succ == 'R':
      return 0


def parse_file(filepath: str):
  workflows = {}
  parsing_workflows = True
  total = 0
  for line in open(filepath):
    line = line.strip()
    if not line:
      parsing_workflows = False
      continue
    if parsing_workflows:
      name, rules = line[:-1].split('{', 1)
      workflows[name] = [parse_rule(r) for r in rules.split(',')]
    else:
      rating = line[1:-1].split(',')
      total += process_rating(workflows, tuple(int(r[2:]) for r in rating))
  return total, workflows


total, workflows = parse_file('inputs/19.test')
print('Part 1:', total)

# TODO: Part 2. If each f the 4 ratings is in range [1, 4000], how many combinations of
# ratings are valid for my workflows?

rating = tuple(slice(1, 4001) for _ in range(4))
print(rating)
print(workflows['in'])

succ = 'in'
num_combos = 0
for name, fn in workflows[succ]:
  if fn is accept:
    if name == 'A':
      num_combos += prod(s.stop - s.start for s in rating)
    elif name == 'R':
      pass
    else:
      succ = name
    break
  # have fn split rating into 2 parts (accept, reject) and recurse
