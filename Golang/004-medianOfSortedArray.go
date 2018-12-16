package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello, playground")
	A := []int{}
	B := []int{1,2,3,4,5}
	res := findMedianSortedArrays(A, B)
	fmt.Println(res)
}

// my solution
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	l1 := len(nums1)
	l2 := len(nums2)

	if l1 == 0 {
		return findMedian(nums2, l2)
	} else if l2 == 0 {
		return findMedian(nums1, l1)
	}
	l := l1+l2
	m := l/2 + 1

	stack := make([]int, 0, m)
	a := 0
	b := 0

	for i := 0; i < m; i++ {

		if b == l2 {
			stack = append(stack, nums1[a])
			a = a + 1
		} else if a == l1 {
			stack = append(stack, nums2[b])
			b = b + 1
		} else if nums1[a] < nums2[b] {
			stack = append(stack, nums1[a])
			a = a + 1
		} else {
			stack = append(stack, nums2[b])
			b = b + 1
		}

	}
	if l%2 != 0 {
		return float64(stack[m-1])
	}
	return float64((stack[m-1] + stack[m-2])) / 2
}

// best
func bestfindMedianSortedArrays(nums1 []int, nums2 []int) float64 {

	var sum = make([]int,0,len(nums1)+len(nums2))
	var i,j int = 0, 0
	for {

		if i >= len(nums1) || j >= len(nums2) {
			for ;i < len(nums1); i++ {
				sum = append(sum, nums1[i])
			}
			for ;j < len(nums2); j++ {
				sum = append(sum, nums2[j])
			}
			break;
		}

		if nums1[i] < nums2[j] {
			sum = append(sum, nums1[i])
			i++
		} else if nums1[i] > nums2[j] {
			sum = append(sum, nums2[j])
			j++
		} else {
			sum = append(sum, nums2[j])
			sum = append(sum, nums1[i])
			i++
			j++
		}
	}

	index := len(sum)/2
	if len(sum) % 2 == 1 {
		return float64(sum[index])
	}

	return float64((float64(sum[index-1]) + float64(sum[index]))/2.0)

}

func findMedian(nums []int, l int) float64 {
	if l == 1 {
		return float64(nums[0])
	}
	m := l / 2
	if l%2 != 0 {
		return float64(nums[m])
	}
	return float64(nums[m-1]+nums[m]) / 2
}
