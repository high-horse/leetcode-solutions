package main

import "fmt"

func main(){
	fmt.Println("hello world")
}

func findDuplicate(nums []int) int {
    hasSeenNumber := make([]int, len(nums))
    for _, num := range nums {
        if hasSeenNumber[num] == -1 {
            return num
        }
        hasSeenNumber[num] = -1
    }
    return -1
}


