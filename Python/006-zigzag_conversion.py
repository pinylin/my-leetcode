# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     006-zigzag_conversion
   Description :
   Author :       pinglin
   date：          18-1-18
-------------------------------------------------
   Change Activity:
                   18-1-18:
-------------------------------------------------
"""
class Solution:
    def convert(self, s, numRows):
        """
        :type s: str
        :type numRows: int
        :rtype: str
        """
        if numRows == 1 or numRows >= len(s):
            return s

        L = [''] * numRows
        index, step = 0, 1

        for x in s:
            L[index] += x
            if index == 0:
                step = 1
            elif index == numRows -1:
                step = -1
            index += step

        return ''.join(L)kj
