// 46.94%  哎
func lengthOfLongestSubstring(s string) int {
	hash := map[byte]int{}
	start := 0
	max := 0
	for i := range s {
		_, ok := hash[s[i]]

		if ok && start <= hash[s[i]] {
			start = hash[s[i]] + 1
		} else {
			if max < i-start+1 {
				max = i - start + 1
			}
		}
		hash[s[i]] = i

	}
	return max
}
