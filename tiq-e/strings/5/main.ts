import {performance} from 'perf_hooks';

function isPalindrome(s: string): boolean {
    // let string = s.replace(/[^a-z0-9]/ig, "").toLowerCase();
    // return string === string.split("").reverse().join("");

    let a1 = s
        .replace(/[^a-z0-9]/ig, "")
        .toLowerCase()
        .split('')

    let a2 = a1.reverse()
    let a3 = a1.map((e, i) => [e, a2[i]])

    return a3.reduce((acc: true | false, [first, second]) => {
        acc = first === second;
        return acc
    }, true)
}


const s = "ab_as";

const start = performance.now();
const result = isPalindrome(s);
const end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
