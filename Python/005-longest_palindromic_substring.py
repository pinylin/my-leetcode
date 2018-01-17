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
# 80
# 这题我是只会暴力的， 下解参见 http://blog.csdn.net/tuobadon/article/details/78989601
class Solution:
    def longestPalindrome(self, s):
        """
        :type s: str
        :rtype: str
        """
        if not s:
            return None
        if len(s) < 2:
            return s
        T = '#'.join('@{}$'.format(s))  # step 1
        # step2
        n = len(T)
        P = [0] * n
        c = 0
        r = 0
        for i in range(1, n - 1):
            i_mirror = c - (i - c)  # i关于中心c的对称位置
            if r > i:  # 利用之前回文串字符对比重复部分
                P[i] = min(r - i, P[i_mirror])
                # 中心扩展法完成之前没有涉及的字符比对
            while T[i + 1 + P[i]] == T[i - 1 - P[i]]:
                P[i] = P[i] + 1
                # 更新当前回文串中心c及终止位置r
            if i + P[i] > r:
                c = i
                r = i + P[i]
                # 找到最大回文半径及对应的回文中心
        maxlen = 0
        centeridx = 0
        for i in range(1, n - 1):
            if P[i] > maxlen:
                maxlen = P[i]
                centeridx = i
                # 获取最长回文串
        begin = (centeridx - maxlen) // 2
        end = (centeridx + maxlen) // 2
        return s[begin:end]


