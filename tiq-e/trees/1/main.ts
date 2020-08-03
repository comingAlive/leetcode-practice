import { performance } from "perf_hooks";

class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;

  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

function maxDepth(root: TreeNode | null): number {
  if (!root) return 0;

  let leftDepth = maxDepth(root.left);
  let rightDepth = maxDepth(root.right);

  if (leftDepth > rightDepth) return leftDepth + 1;
  else return rightDepth + 1
}

const root = new TreeNode(5, null, null);

const start = performance.now();
const result = maxDepth(root);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
