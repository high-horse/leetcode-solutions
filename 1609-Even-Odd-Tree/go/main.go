

import (
    "container/list"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isEvenOddTree(root *TreeNode) bool {
    if root == nil {
        return true
    }

    queue := list.New()
    queue.PushBack(root)

    level := 0
    for queue.Len() != 0 {
        n := queue.Len()
        cur := 0
        for i := 0; i < n; i++ {
            e := queue.Front()
            node := e.Value.(*TreeNode)

            if level%2 == 0 && ((cur != 0 && cur >= node.Val) || node.Val%2 == 0 ) {
                return false
            }
            if level%2 == 1 && ((cur != 0 && cur <= node.Val) || node.Val%2 == 1 ) {
                return false
            }

            cur = node.Val
        
            if node.Left != nil {
                queue.PushBack(node.Left)
            }
            if node.Right != nil {
                queue.PushBack(node.Right)
            }
            queue.Remove(e)
        }
        level++
    }
    return true
}


func main(){}
