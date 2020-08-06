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

function levelOrder(root: TreeNode | null): number[][] {
  return bfs([root], [], 0);
}

function bfs(
  que: Array<TreeNode | null>,
  res: number[][],
  depth: number
): number[][] {
  if (!que.length) return res;
  const newQ: Array<TreeNode | null> = [];
  que.forEach((node: TreeNode | null) => {
    if (node) {
      if (!res[depth]) res[depth] = [];
      res[depth].push(node.val);
      newQ.push(node.left);
      newQ.push(node.right);
    }
  });
  return bfs(newQ, res, depth + 1);
}

const root = new TreeNode(5, null, null);

const start = performance.now();
const result = levelOrder(root);
const end = performance.now();
console.log("Duration:", end - start);
console.log("Result:", result);
