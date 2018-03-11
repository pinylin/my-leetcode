# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     013-roman_to_int
   Description :
   Author :       pinglin
   date：          18-2-6
-------------------------------------------------
   Change Activity:
                   18-2-6:
-------------------------------------------------
"""
class Solution:
    def romanToInt(self, s):
        """
        :type s: str
        :rtype: int
        """
        d = {'M':1000, 'D':500, 'C':100, 'L':50, 'X':10, 'V':5, 'I':1}
        res, p = 0, 'I'
        for c in s[::-1]:
            res, p = res - d[c] if d[c] < d[p] else res + d[c], c
        return res
