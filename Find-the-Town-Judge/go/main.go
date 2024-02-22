package main

import "fmt"

func findJudge(n int, trust [][]int) int {
    trustCounts := make([]int, n+1)

    for _, t := range trust {
        trustCounts[t[0]]--
        trustCounts[t[1]]++
    }

    for i := 1; i <= n; i++ {
        if trustCounts[i] == n-1 {
            return i
        }
    }

    return -1
}

func main() {
    trust := [][]int{{1, 2}}
    n := 2

    answer := findJudge(n, trust)
    fmt.Println(answer)
}
