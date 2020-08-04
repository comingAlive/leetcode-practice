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

function isValidBST(
  root: TreeNode,
  min: TreeNode | null = null,
  max: TreeNode | null = null
): boolean {
  if (!root) return true;

  if (min && root.val <= min!.val) return false;
  if (max && root.val >= max!.val) return false;

  return (
    isValidBST(root.left as TreeNode, min, root) &&
    isValidBST(root.right as TreeNode, root, max)
  );
}

const root = new TreeNode(5, null, null);

const start = performance.now();
const result = isValidBST(root);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
