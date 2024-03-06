package main

import(
	"fmt"
)

type ListNode struct {
    Val  int
    Next *ListNode
}

func hasCycle(head *ListNode) bool {
    if head == nil || head.Next == nil {
        return false
    }

    seen := make(map[*ListNode]bool)
    cur := head

    for cur != nil {
        if _, exists := seen[cur]; exists {
            return true
        }

        seen[cur] = true
        cur = cur.Next
    }

    return false
}

func createLinkedList(values []int, pos int) *ListNode {
    var head, curr *ListNode
    nodes := make([]*ListNode, len(values))

    for i, val := range values {
        node := &ListNode{Val: val}
        nodes[i] = node

        if i == 0 {
            head = node
        } else {
            curr.Next = node
        }

        curr = node
    }

    // Create a cycle if pos is valid
    if pos >= 0 && pos < len(values) {
        curr.Next = nodes[pos]
    }

    return head
}

func main() {
    values := []int{3, 2, 0, -4}
    pos := 1

    head := createLinkedList(values, pos)
    hasCycle(head)
}
