package main

import (
    "fmt"
)

func maxFrequencyElements(nums []int) int {
    // Create a map to store frequency of elements
    freqMap := make(map[int]int)

    // Populate frequency map
    for _, val := range nums {
        freqMap[val]++
    }

    // Find maximum frequency
    max := -1
    for _, freq := range freqMap {
        if freq > max {
            max = freq
        }
    }

    // Calculate count of elements with maximum frequency
    count := 0
    for _, freq := range freqMap {
        if freq == max {
            count += max
        }
    }

    return count
}

func main() {
    nums := []int{1, 2, 2, 3, 3, 3}
    result := maxFrequencyElements(nums)
    fmt.Println("Result:", result)
}
