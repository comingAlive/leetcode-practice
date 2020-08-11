import { performance } from "perf_hooks";

class ListNode {
  val: number;
  next: ListNode | null;

  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function isPalindrome(head: ListNode | null): boolean {
  let curr = head;
  const traverse = (node: ListNode | null): boolean => {
    if (node === null) return true;

    const prevIsSame = traverse(node.next);

    const currIsSame = curr!.val === node!.val;
    curr = curr!.next;
    return prevIsSame && currIsSame;
  };
  return traverse(head);
}

const head = new ListNode(1, null);

const start = performance.now();
const result = isPalindrome(head);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
