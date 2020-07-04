package main

import "fmt"

/* my
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
*/

// best
func maxPoints(points [][]int) int {
	if len(points) <= 2 {
		return len(points)
	}

	var count, same int

	for i := 0; i < len(points)-2; i++ {
		same = 1 // 记录直线上与i点相同点的个数，包含i点本身。初始就只有i点一个。
		for j := i + 1; j < len(points); j++ {
			if points[i][0] == points[j][0] && points[i][1] == points[j][1] { // i,j 重合了
				same++
				continue
			} else {
				max := same + 1 // 此时直线上增加一个点，即j点
				for k := j + 1; k < len(points); k++ {
					// 判断三点是否成一线，将除法转成乘法
					if (points[j][1]-points[i][1])*(points[j][0]-points[k][0]) == (points[j][1]-points[k][1])*(points[j][0]-points[i][0]) {
						max++
					}
				}

				if count < max {
					count = max
				}
			}
		}
		// 因为有可能所有的点都相同，此时不会执行第三个for循环，就无法对count赋值
		if count < same {
			count = same
		}
	}

	return count
}

func isSameLine(p1, p2, p3 []int) bool {
	return (p3[0]-p1[0])*(p2[1]-p1[1]) == (p2[0]-p1[0])*(p3[1]-p1[1])
}

func main() {
	//points := [][]int{
	//	[]int{3, 1},
	//	[]int{12, 3},
	//	[]int{3, 1},
	//	[]int{-6, -1},
	//}
	//res := maxPoints(points)
	res := make([]int, 0, 3)
	fmt.Println(res)
}
