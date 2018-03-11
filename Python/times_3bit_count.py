# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     times_3bit_count
   Description :
   Author :       pinglin
   date：          18-2-28
-------------------------------------------------
   Change Activity:
                   18-2-28:
-------------------------------------------------
"""

def solution(A):
    res = 0
    ret = []
    for item in A:
        res += (2 ** item) * 3
    while res > 0:
        res, loc = trans(res)
        ret.insert(0, loc)
    return ret

def trans(res):
    loc = len(bin(res)) - 3
    temp = res - 2 ** loc
    if temp > 0:
        return temp, loc
    return -1, loc

print(solution([1, 4, 5]))
print(solution([0, 4, 5]))
print(solution([]))
