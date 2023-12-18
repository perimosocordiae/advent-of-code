// Usage: ts-node day15.ts

function hashAlg(s: string): number {
  let hash = 0;
  for (let i = 0; i < s.length; i++) {
    hash += s.charCodeAt(i);
    hash *= 17;
    hash %= 256;
  }
  return hash;
}

const text = require('fs').readFileSync('inputs/15.full', 'utf-8');
const initSeq = text.trim().split(',');
const part1 = initSeq.map(hashAlg)
  .reduce((a: number, b: number) => a + b, 0);
console.log('Part 1:', part1);

const boxes: Array<Map<string, number>> = [];
for (let i = 0; i < 256; i++) {
  boxes.push(new Map());
}
for (const step of initSeq) {
  if (step.endsWith('-')) {
    const label = step.slice(0, -1);
    const box = boxes[hashAlg(label)];
    box.delete(label);
  } else {
    const [label, num] = step.split('=');
    const box = boxes[hashAlg(label)];
    box.set(label, parseInt(num, 10));
  }
}

let totalPower = 0;
for (let i = 0; i < 256; i++) {
  const box = boxes[i];
  let j = 1;
  for (const num of box.values()) {
    totalPower += (i + 1) * j * num;
    j++;
  }
}
console.log('Part 2:', totalPower);
