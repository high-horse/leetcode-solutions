/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
// Function to calculate the diameter of a binary tree
func diameterOfBinaryTree(root *TreeNode) int {
    var res = [1]int{0}

    var dfs func(node *TreeNode) int
    dfs = func(node *TreeNode) int {
        if node == nil {
            return -1
        }

        leftHeight := dfs(node.Left)
        rightHeight := dfs(node.Right)

        res[0] = max(res[0], 2+leftHeight+rightHeight)

        return 1 + max(leftHeight, rightHeight)
    }

    dfs(root)
    return res[0]
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
