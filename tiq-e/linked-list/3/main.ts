import {performance} from "perf_hooks";

class ListNode {
    val: number;
    next: ListNode | null;

    constructor(val?: number, next?: ListNode | null) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

function reverseList(head: ListNode | null): ListNode | null {
    let pre = null;
    while (head) {
        const next = head.next;
        head.next = pre;
        pre = head;
        head = next;
    }
    return pre;
}

const head = new ListNode(1, null);

const start = performance.now();
const result = reverseList(head);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
