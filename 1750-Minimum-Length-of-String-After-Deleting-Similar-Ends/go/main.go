package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello World")
	s := "aabccabba"
	fmt.Println(minimumLength(s))

}

func minimumLength(s string) int {

	left := 0
	right := len(s) - 1
	// fmt.Println(s[0])

	for left < right {
		if s[left] == s[right] {
			same := s[left]
			for left <= right && s[left] == same {
				left += 1
			}
			for left <= right && s[right] == same {
				right -= 1
			}
		} else {
			break
		}
	}
	return right - left + 1
}
