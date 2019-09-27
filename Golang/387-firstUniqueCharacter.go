/*
-----------------------
  @Time :             2019/9/13 1:28 PM 
  @Author :           pinglin
  @File :             387-firstUniqueCharacter
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2019/9/13 1:28 PM
                
*/
package main

import "fmt"

func firstUniqChar(s string) int {
	rec := make([]int, 26)
	for _, b := range s {
		rec[b-'a']++
	}

	for i, b := range s {
		if rec[b-'a'] == 1 {
			return i
		}
	}

	return -1
}

func main(){
	fmt.Print(firstUniqChar("loveleetcode"))
}
