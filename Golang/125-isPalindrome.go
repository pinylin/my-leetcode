/*
-----------------------
  @Time :             2019/9/13 7:20 AM 
  @Author :           pinglin
  @File :             125-isPalindrome
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2019/9/13 7:20 AM
                
*/
package main

import (
	"fmt"
	"strings"
)

func isPalindrome(s string) bool {
	s = strings.ToLower(s)
	str := ""
	for _, r := range s {
		if r >= 'a' && r <= 'z' || r >= '0' && r <= '9' {
			str += string(r)
		}
	}
	head, tail := 0, len(str)-1
	for head < tail {
		if str[head] != str[tail] {
			return false
		}
		head++
		tail--
	}
	return true
}

func main() {
	fmt.Println(isPalindrome("OPPO"))
}