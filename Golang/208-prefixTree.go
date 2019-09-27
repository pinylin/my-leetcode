/*
-----------------------
  @Time :             2019/9/13 11:54 AM 
  @Author :           pinglin
  @File :             208-prefixTree
  @Software:          GoLand
-----------------------
  Change Activity:    
                      2019/9/13 11:54 AM
                
*/
package main

import "fmt"

const MAXCAP = 26

type Trie struct {
	isWord   bool
	children map[rune]*Trie
}

/** Initialize your data structure here. */
func Constructor() Trie {
	root := new(Trie)
	root.children = make(map[rune]*Trie, MAXCAP)
	root.isWord = false
	return *root
}

/** Inserts a word into the trie. */
func (this *Trie) Insert(word string) {
	cur := this
	for i, r := range word {
		if cur.children[r] == nil {
			node := new(Trie)
			node.children = make(map[rune]*Trie, MAXCAP)
			cur.children[r] = node

		}
		cur = cur.children[r]
		if i == len(word)-1 {
			cur.isWord = true
		}

	}
}

/** Returns if the word is in the trie. */
func (this *Trie) Search(word string) bool {
	cur := this
	for _, r := range word {
		if cur.children[r] == nil {
			return false
		}
		cur = cur.children[r]
	}
	return cur.isWord
}

/** Returns if there is any word in the trie that starts with the given prefix. */
func (this *Trie) StartsWith(prefix string) bool {
	cur := this
	for _, r := range prefix {
		if cur.children[r] == nil {
			return false
		}
		cur = cur.children[r]
	}
	return true
}



func main(){
	t := Constructor()
	t.Insert("Hello")
	fmt.Print(t.Search("Hello"),"\n")
	fmt.Print(t.Search("Hallo"),"\n")
}