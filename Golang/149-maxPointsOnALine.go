package main

import (
	"fmt"
)

func maxPoints(points [][]int) int {
	n := len(points)
	// diffMap 用来过滤掉相同的点，并记录他们的个数
	diffMap := make(map[string]int, n)

	var ps [][]int
	for i := 0; i < n; i++ {
		key := fmt.Sprintf("%v/%v", points[i][0], points[i][1])
		if _, ok := diffMap[key]; !ok {
			ps = append(ps, points[i])
		}
		diffMap[key]++
	}
	points = ps
	size := len(points)

	if size <= 2 {
		return n
	}

	max := 0

	for i := 0; i < size-1; i++ {
		for j := i + 1; j < size; j++ {
			count := diffMap[fmt.Sprintf("%v/%v", points[i][0], points[i][1])] + diffMap[fmt.Sprintf("%v/%v", points[j][0], points[j][1])]

			// 所有的点，都要检查，是否与 i，j 共线
			for k := 0; k < size; k++ {
				if k == i || k == j {
					continue
				}
				if isSameLine(points[i], points[j], points[k]) {
					count += diffMap[fmt.Sprintf("%v/%v", points[k][0], points[k][1])]
				}
			}

			if max < count {
				max = count
			}
		}
	}

	return max
}

func isSameLine(p1, p2, p3 []int) bool {
	return (p3[0]-p1[0])*(p2[1]-p1[1]) == (p2[0]-p1[0])*(p3[1]-p1[1])
}

func main() {
	points := [][]int{
		[]int{3, 1},
		[]int{12, 3},
		[]int{3, 1},
		[]int{-6, -1},
	}
	res := maxPoints(points)
	fmt.Println(res)
}
