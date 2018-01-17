# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     004-median_of_two_sorted_array
   Description :
   Author :       pinglin
   date：          18-1-17
-------------------------------------------------
   Change Activity:
                   18-1-17:
-------------------------------------------------
"""
# 69.41

class Solution:
    def findMedianSortedArrays(self, nums1, nums2):
        """
        :type nums1: List[int]
        :type nums2: List[int]
        :rtype: float
        """
        res = sorted(nums1 + nums2)
        size = len(res)
        if size == 1:
            return res[0]
        mid = size // 2
        if size % 2:
            return res[mid]
        else:
            return (res[mid - 1] + res[mid]) / 2