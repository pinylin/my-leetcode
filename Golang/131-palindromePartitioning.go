/*
-----------------------
  @Time :             2019/9/13 9:17 AM 
  @Author :           pinglin
  @File :             131-palindromePartitioning
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2019/9/13 9:17 AM
                
*/
package main

import (
	"fmt"
)
var res [][]string

func partition(s string) [][]string {
	helper(s, 0, []string{})
	return res
}

func helper(s string, start int, tmp []string) {
	if start == len(s) {
		m := make([]string, len(tmp))
		copy(m, tmp)
		res = append(res, m)
		return
	}
	for i := start; i < len(s); i++ {
		if isPalindrome_131(s, start, i) {
			tmp = append(tmp, s[start:i+1])
			helper(s, i+1, tmp)
			tmp = tmp[:len(tmp)-1]
		}
	}
}

func isPalindrome_131(s string, start, end int) bool {
	for start < end {
		if s[start] != s[end] {
			return false
		}
		start++
		end--
	}
	return true
}


func main() {
	fmt.Println(partition("a"))
}