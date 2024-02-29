/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function isEvenOddTree(root: TreeNode | null): boolean {
    if (root === null) {
        return true;
    }

    let queue: Array<TreeNode> = [root];
    let level = 0;

    while (queue.length !== 0) {
        let n = queue.length;
        let cur = 0;

        for (let i = 0; i < n; i++) {
            let node = queue.shift()!;

            if (level % 2 === 0 && ((cur !== 0 && cur >= node.val) || node.val % 2 === 0)) {
                return false;
            }
            if (level % 2 === 1 && ((cur !== 0 && cur <= node.val) || node.val % 2 === 1)) {
                return false;
            }

            cur = node.val;

            if (node.left !== null) {
                queue.push(node.left);
            }
            if (node.right !== null) {
                queue.push(node.right);
            }
        }
        level++;
    }
    return true;
}
