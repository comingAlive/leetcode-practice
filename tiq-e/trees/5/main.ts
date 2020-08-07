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

function sortedArrayToBST(nums: number[]): TreeNode | null {
  if (nums.length === 0) return null;
  if (nums.length === 1) return new TreeNode(nums[0]);

  let mid = Math.floor(nums.length / 2);

  let root = new TreeNode(nums[mid]);
  root.left = sortedArrayToBST(nums.slice(0, mid));
  root.right = sortedArrayToBST(nums.slice(mid + 1));

  return root;
}

const root = [-10, -3, 0, 5, 9];

const start = performance.now();
const result = sortedArrayToBST(root);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
