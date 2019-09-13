/*
-----------------------
  @Time :             2019/9/13 11:42 AM 
  @Author :           pinglin
  @File :             139-wordBreak
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2019/9/13 11:42 AM
                
*/
package main

import "fmt"

func wordBreak(s string, wordDict []string) bool {
	result := make(map[string]bool)
	return helper(s, wordDict, result)
}
func helper(s string, wordDict []string, result map[string]bool) bool {
	if len(s) == 0 {
		return true
	}

	if ret, ok := result[s]; ok {
		return ret
	}

	for _, word := range wordDict {
		if len(word) > len(s) || word != s[:len(word)] {
			continue
		}

		if helper(s[len(word):], wordDict, result) {
			result[s] = true
			return true
		}
	}

	result[s] = false
	return false
}

func main() {
	// s = "applepenapple", wordDict = ["apple", "pen"]
	fmt.Println(wordBreak("applepenapple", []string{"apple", "pen"}))
}