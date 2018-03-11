# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     011-container_with_most_water
   Description :
   Author :       pinglin
   date：          18-2-4
-------------------------------------------------
   Change Activity:
                   18-2-4:
-------------------------------------------------
"""


class Solution:
    def maxArea(self, height):
        """
        :type height: List[int]
        :rtype: int
        """
        l, r, width, res = 0, len(height) - 1, len(height) - 1, 0
        for w in range(width, 0, -1):
            if height[l] < height[r]:
                res, l = max(res, height[l] * w), l + 1
            else:
                res, r = max(res, height[r] * w), r - 1

        return res
