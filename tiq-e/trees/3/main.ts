import {performance} from "perf_hooks";

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

function isSymmetric(root: TreeNode | null): boolean {
  if (root === null) return true;

  return isMirror(root.left, root.right);
}
function isMirror(tree1: TreeNode | null, tree2: TreeNode | null): boolean {
  if (tree1 === null && tree2 === null) return true;
  if (tree1 === null || tree2 === null) return false;
  if (tree1.val !== tree2.val) return false;

  return isMirror(tree1.left, tree2.right) && isMirror(tree1.right, tree2.left);
}


const root = new TreeNode(5, null, null);

const start = performance.now();
const result = isSymmetric(root);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
