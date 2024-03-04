package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	fmt.Println("Hello World")
	tokens := []int{100, 200, 300, 400}
	power := 200
	fmt.Println(bagOfTokensScore(tokens, power))
}

func bagOfTokensScore(tokens []int, power int) int {
	sort.Ints(tokens)

	left := 0
	right := len(tokens) - 1

	score := 0
	max_score := 0

	for left <= right {
		if tokens[left] <= power {
			power -= tokens[left]
			left += 1
			score += 1
			max_score = int(math.Max(float64(max_score), float64(score)))
		} else if score > 0 {
			power += tokens[right]
			right -= 1
			score -= 1
		} else {
			break
		}
	}

	return max_score
}
