import { performance } from "perf_hooks";

class ListNode {
  val: number;
  next: ListNode | null;

  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function mergeTwoLists(
  l1: ListNode | null,
  l2: ListNode | null
): ListNode | null {
  let head = null;

  if (l1 == null) return l2;
  if (l2 == null) return l1;

  if (l1.val < l2.val) {
    head = l1;
    l1 = l1.next;
  } else {
    head = l2;
    l2 = l2.next;
  }
  head.next = mergeTwoLists(l1, l2);
  return head;
}

const first = new ListNode(1, null);
const second = new ListNode(1, null);

const start = performance.now();
const result = mergeTwoLists(first, second);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
