import {performance} from 'perf_hooks';

function isAnagram(s: string, t: string): boolean {
    return s.split('')
            .sort()
            .join('')
        === t.split('')
            .sort()
            .join('')
};

const s = "anagram";
const t = "nagaram";

const start = performance.now();
const result = isAnagram(s, t);
const end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
