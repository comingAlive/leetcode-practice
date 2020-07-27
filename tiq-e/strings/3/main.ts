import {performance} from 'perf_hooks';
import {log} from "util";

function firstUniqChar(s: string): number {
    // for (let i = 0; i < s.length; i++) {
    //     if (s.indexOf(s[i]) === s.lastIndexOf(s[i])) {
    //         return i;
    //     }
    // }
    // return -1;
    let m = new Map()
    for (let i = 0; i < s.length; i++) {
        if (m.has(s[i])) {
            m.set(s[i], m.get(s[i])+ 1)
        } else if (!m.has(s[i])) {
            m.set(s[i], 1);
        }
    }
    for (let i = 0; i < s.length; i++) {
        if (m.get(s[i]) === 1) {
            return i
        }
    }
    return -1
};

let s = "loveleetcode"

let start = performance.now();
let result = firstUniqChar(s);
let end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
