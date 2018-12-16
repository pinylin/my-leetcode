/*
-----------------------
  @Time :             2018/12/16 8:37 AM 
  @Author :           pinglin
  @File :             011-containerWithMostWater
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2018/12/16 8:37 AM
                
*/
package main

import "fmt"

func main() {
	fmt.Println("Hello, playground")
	A := []int{1,2,3,4,5}
	res := maxArea(A)
	fmt.Println(res)
}
// Runtime: 16 ms, faster than 100.00% of Go online submissions for Container With Most Water.
func maxArea(height []int) int {
	lh := len(height)
	l, r, w, res := 0, lh-1, lh-1, 0
	for i:=0;i < lh;i++{
		if height[l] < height[r]{
			temp := height[l] * w
			if temp > res{
				res = temp
			}
			l = l + 1
			w = w - 1
		}else {
			temp := height[r] * w
			if temp > res{
				res = temp
			}
			r = r - 1
			w = w - 1
		}
	}

	return res
}