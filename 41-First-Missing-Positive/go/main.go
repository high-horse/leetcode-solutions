package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello World")
    nums := []int{
        7,8,9,11,12,
    }
    fmt.Println(firstMissingPositive(nums))
}

func firstMissingPositive(nums []int) int {
	n := len(nums)

	for i:=0; i< n; i++ {
		if (nums[i] >= 1 &&
			nums[i] <= n &&
			nums[i] != nums[nums[i]-1]) {
			nums[i], nums[nums[i]-1] = nums[nums[i]-1], nums[i]
			i -= 1
		}
	}

	for i := range n {
		if nums[i] != i+1 {
			return i + 1
		}
	}
	return n + 1
}
