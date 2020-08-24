import {performance} from "perf_hooks";

function isValid(s: string): boolean {
    let stack = [];
    for (const char of s) {
        switch (char) {
            case "(":
                stack.push(")");
                break;
            case "[":
                stack.push("]");
                break;
            case "{":
                stack.push("}");
                break;
            default:
                if (char !== stack.pop()) {
                    return false;
                }
        }
    }
    return !stack.length;
}

const x = "()[]{}";

const start = performance.now();
const result = isValid(x);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
