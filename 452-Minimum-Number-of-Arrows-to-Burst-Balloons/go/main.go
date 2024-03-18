package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	fmt.Println("Hello World")
	points := [][]int{
		{10, 16},
		{2, 8},
		{1, 6},
		{7, 12},
	}

	arrows := findMinArrowShots(points)
	fmt.Println(arrows)
}

func findMinArrowShots(points [][]int) int {
	sort.Slice(points, func(i, j int) bool {
		return points[i][1] < points[j][1]
	})

	min := math.MinInt
	end := min
	arrows := 0

	for _, i := range points {
		if i[0] == min {
			if i[1] > end {
				arrows += 1
				end = i[1]
			}
		} else if i[0] > end {
			arrows += 1
			end = i[1]
		}
	}
	return arrows
}
