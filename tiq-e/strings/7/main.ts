import { performance } from "perf_hooks";

function strStr(haystack: string, needle: string): number {
  if (needle === "") return 0;
  let l = needle.length;

  for (let i = 0; i < haystack.length - needle.length + 1; i++) {
    if (haystack.slice(i, i + l) == needle) return i;
  }
  return -1;
}

const haystack = "hello";
const needle = "ll";

const start = performance.now();
const result = strStr(haystack, needle);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
