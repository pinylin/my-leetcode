# 91.36 135ms
class Solution:
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        start = maxLen = 0
        appeard = {}
        for i in range(len(s)):
            if s[i] in appeard and start <= appeard[s[i]]:
                start = appeard[s[i]] + 1
            else:
                maxLen = max(maxLen, i - start +1)

            appeard[s[i]] = i
        return maxLen
