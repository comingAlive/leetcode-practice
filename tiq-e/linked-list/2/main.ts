import { performance } from "perf_hooks";

class ListNode {
  val: number;
  next: ListNode | null;

  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
  if (head == null) return null;

  const h0 = new ListNode(0, head);
  let n1: ListNode | null = h0;
  let n2: ListNode | null = h0;

  for (let i = 1; i <= n + 1; i++) {
    n1 = n1.next!;
  }

  while (n1 != null) {
    n1 = n1.next!;
    n2 = n2.next!;
  }

  n2.next = n2.next!.next;
  return h0.next;
}

const head = new ListNode(1, null);

const start = performance.now();
const result = removeNthFromEnd(head, 1);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
