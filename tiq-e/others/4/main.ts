import {performance} from "perf_hooks";

function generate(numRows: number): number[][] {
    if (numRows === 0) return [];

    let rows: number[][] = [[1]];

    for (let i = 1; i < numRows; i++) {
        let newRow = [1];
        for (let j = 1; j <= i; j++)
            newRow.push(rows[i - 1][j - 1] + (rows[i - 1][j] || 0));
        rows.push(newRow);
    }
    return rows;
}

const x = 5;

const start = performance.now();
const result = generate(x);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
