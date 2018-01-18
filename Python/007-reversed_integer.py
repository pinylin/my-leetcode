# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     007-reversed_integer
   Description :
   Author :       pinglin
   date：          18-1-18
-------------------------------------------------
   Change Activity:
                   18-1-18:
-------------------------------------------------
"""


class Solution:
    def reverse(self, x):
        """
        :type x: int
        :rtype: int
        """

        neg = 1 if x >= 0 else -1
        x = neg * x

        y = int(str(x)[::-1])

        if y > 0x7FFFFFFF:
            return 0

        return neg * y
