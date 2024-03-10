package main

import "fmt"

func intersection(nums1 []int, nums2 []int) []int {
    m := make(map[int]bool)
    var res []int

    for _, num := range nums1 {
        m[num] = true
    }

    for _, num := range nums2 {
        if m[num] {
            res = append(res, num)
            delete(m, num)
        }
    }

    return res
}

func main() {
    nums1 := []int{1, 2, 2, 1}
    nums2 := []int{2, 2}
    fmt.Println(intersection(nums1, nums2))
}

