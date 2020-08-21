import {performance} from "perf_hooks";

function romanToInt(s: string): number {
    let map = new Map(),
        int = 0,
        i = 0;
    map.set("I", 1);
    map.set("V", 5);
    map.set("X", 10);
    map.set("L", 50);
    map.set("C", 100);
    map.set("D", 500);
    map.set("M", 1000);

    while (i < s.length) {
        const currentValue = map.get(s[i]);
        if (currentValue < map.get(s[i + 1])) int -= currentValue;
        else int += currentValue;

        i++;
    }
    return int;
}

const n = "IV";

const start = performance.now();
const result = romanToInt(n);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
