import {performance} from 'perf_hooks';


function myAtoi(str: string): number {
    let filter = '0123456789+- '
    let res = 0
    let sign = 1
    for (let char of str) {
        let index = filter.indexOf(char)
        if (index != -1) {
            if (char == " ") continue
            if (filter[10] == "+") filter = filter.slice(0, 10)
            if (char == "+") continue
            if (char == "-") {
                sign = -sign;
                continue
            }
            res = res * 10 + index
        } else {
            break;
        }
    }
    res = res * sign
    if (res > 2 ** 31 - 1) res = 2 ** 31 - 1
    else if (res < -(2 ** 31)) res = -(2 ** 31)
    return res
}


const s = "42";

const start = performance.now();
const result = myAtoi(s);
const end = performance.now();
console.log('Duration:', end - start);
console.log('Result:', result)
