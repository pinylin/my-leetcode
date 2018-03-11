# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     012-int_to_roman
   Description :
   Author :       pinglin
   date：          18-2-5
-------------------------------------------------
   Change Activity:
                   18-2-5:
-------------------------------------------------
"""


class Solution:
    def intToRoman(self, num):
        """
        :type num: int
        :rtype: str
        """
        values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]
        numerals = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"]
        res = ""

        for n, v in zip(numerals, values):
            res += (num // v) * n
            num %= v
        return res