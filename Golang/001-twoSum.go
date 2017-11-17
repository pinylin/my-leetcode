package main

//1 暴力搜索
/*
func twoSum(nums []int, target int) []int {
	len := len(nums)
	for i := 0; i < len; i++ {
		for j := i + 1; j < len; j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return []int{}
}
*/
//2 Hash
//  9ms  61.93%
func twoSum(nums []int, target int) []int {
	len := len(nums)
	indexs := make([]int, 2)
	hash := map[int]int{}
	for i := 0; i < len; i++ {
		index, ok := hash[target-nums[i]]
		if ok {
			if i == index {
				continue
			}
			indexs[0] = index
			indexs[1] = i
			break
		} else {
			hash[nums[i]] = i
		}

	}
	return indexs
}
