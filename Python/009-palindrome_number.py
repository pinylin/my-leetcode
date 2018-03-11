# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     009-palindrome_number
   Description :
   Author :       pinglin
   date：          18-1-31
-------------------------------------------------
   Change Activity:
                   18-1-31:
-------------------------------------------------
"""
import math
class Solution:
    def isPalindrome(self, x):
        """
        :type x: int
        :rtype: bool
        """
        if x < 0:
            return False
        ss = str(x)
        size = len(ss)
        for i in range(math.ceil(size/2 )):
            if ss[i] == ss[-i-1]:
                pass
            else:
                return False
        return True