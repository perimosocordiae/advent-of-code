// Usage: node day13.js

function count_mismatches(arr1, arr2) {
  return arr1.filter((val, idx) => val !== arr2[idx]).length;
}

function reflect_row(arr, num_mismatches) {
  const nr = arr.length;
  for (let i = 1; i < nr; i++) {
    const n = Math.min(i, nr - i);
    const lhs = arr.slice(i - n, i);
    const rhs = arr.slice(i, i + n);
    const m = lhs.map((val, idx) => count_mismatches(val, rhs[n - idx - 1]))
                 .reduce((a, b) => a + b, 0);
    if (m === num_mismatches) {
      return i;
    }
  }
  return 0;
}

function solve(chunk, num_mismatches) {
  const arr = chunk.split('\n').map(line => [...line].map(c => '.#'.indexOf(c)));
  const arr_t = arr[0].map((_, i) => arr.map(row => row[i]));
  return 100 * reflect_row(arr, num_mismatches) + reflect_row(arr_t, num_mismatches);
}

const text = require('fs').readFileSync('inputs/13.full', 'utf-8');
const chunks = text.trim().split('\n\n');
console.log('Part 1:', chunks.reduce((sum, chunk) => sum + solve(chunk, 0), 0));
console.log('Part 2:', chunks.reduce((sum, chunk) => sum + solve(chunk, 1), 0));
