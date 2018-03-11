# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     max_slice_sum
   Description :
   Author :       pinglin
   date：          18-2-28
-------------------------------------------------
   Change Activity:
                   18-2-28:
-------------------------------------------------
"""
def solution(A):

    l, r, width, res = 0, len(A) - 1, len(A) - 1, 0
    res = sum(A)
    for w in range(width):

        if A[l] < A[r]:
            res, l = max(res, sum(A[l:r])), l + 1
        else:
            res, r = max(res, sum(A[l:r])), r - 1
    return res

print(solution([10, 1, -1, -9, 2, -2, -3, 13]))
print(solution([10,-4, 2, -10, 9]))
print(solution([-1, -2, -3, -3, -2, -1]))
print(solution([-10, -2, -3, -3, -2, -1]))
