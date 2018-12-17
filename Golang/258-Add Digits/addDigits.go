/*
-----------------------
  @Time :             2018/12/17 10:21 PM 
  @Author :           pinglin
  @File :             addDigits
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2018/12/17 10:21 PM
                
*/
package main

import (
	"fmt"
)

func main() {
	num := 322322455
	fmt.Println(addDigits(num))
}


func addDigits(num int) int {
	return (num-1)%9 + 1
}