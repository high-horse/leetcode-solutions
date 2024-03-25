package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println("Hello World")
	nums := []int{
		4, 3, 2, 7, 8, 2, 3, 1,
	}
	fmt.Println("duplicates =>", findDuplicates(nums))
}

func findDuplicates(nums []int) []int {
	var duplicates []int

	for _, num := range nums {
		index := int(math.Abs(float64(num))) - 1

		if nums[index] < 0 {
			duplicates = append(duplicates, int(math.Abs(float64(num))))
		} else {
			nums[index] = -nums[index]
		}
	}
	return duplicates
}
