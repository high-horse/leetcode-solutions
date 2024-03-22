package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello World")
	first := ListNode{
		Val:  1,
		Next: nil,
	}

	second := ListNode{
		Val:  2,
		Next: &first,
	}
	third := ListNode{
		Val:  2,
		Next: &second,
	}
	head := ListNode{
		Val:  1,
		Next: &third,
	}

	fmt.Println(isPalindrome(&head))

}

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	var vecs []int
	for head != nil {
		fmt.Println(head.Val)
		vecs = append(vecs, head.Val)
		head = head.Next
	}

	first := 0
	length := len(vecs)
	last := length - 1

	for i := 0; i < length/2; i++ {
		if vecs[first] != vecs[last] {
			return false
		}
		first++
		last--
	}
	return true
}

