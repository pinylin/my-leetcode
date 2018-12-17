/*
-----------------------
  @Time :             2018/12/17 9:41 PM 
  @Author :           pinglin
  @File :             maxAreaOfIsland.go
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2018/12/17 9:41 PM
                
*/
package main

import "fmt"

func main() {
	grid := [][]int{
		{1,1,0,0,0},
		{1,1,0,0,0},
		{0,0,0,1,1},
		{0,0,0,1,1}}
	fmt.Println(maxAreaOfIsland(grid))
}

func maxAreaOfIsland(grid [][]int) int {
	maxArea := 0
	for i := range grid {
		for j := range grid[i] {
			maxArea = max(maxArea, getArea(grid, i, j))
		}
	}
	return maxArea
}

// 返回与 [i,j] 处联通的岛的面积
func getArea(grid [][]int, i, j int) int {
	if grid[i][j] == 0 {
		return 0
	}

	grid[i][j] = 0
	area := 1

	if i != 0 {
		area += getArea(grid, i-1, j)
	}

	if j != 0 {
		area += getArea(grid, i, j-1)
	}

	if i != len(grid)-1 {
		area += getArea(grid, i+1, j)
	}

	if j != len(grid[0])-1 {
		area += getArea(grid, i, j+1)
	}

	return area
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
