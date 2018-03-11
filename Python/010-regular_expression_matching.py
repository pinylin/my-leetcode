# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     010-regular_expression_matching
   Description :
   Author :       pinglin
   date：          18-2-1
-------------------------------------------------
   Change Activity:
                   18-2-1:
-------------------------------------------------
"""

class Solution(object):
    def isMatch(self, s, p):
        prev = [False, True]
        for j in range(len(p)):
            prev.append(p[j] == '*' and prev[j])

        for i in range(len(s)):
            curr = [False, False]
            for j in range(len(p)):
                if p[j] == '*':
                    curr.append(curr[j] or curr[j + 1]
                                or (prev[j + 2] and p[j - 1] in (s[i], '.')))
                else:
                    curr.append(prev[j + 1] and p[j] in (s[i], '.'))
            prev = curr
        return prev[-1]

