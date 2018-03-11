# -*- coding: utf-8 -*-
"""
-------------------------------------------------
   File Name：     014-longest_common_prefix
   Description :
   Author :       pinglin
   date：          18-2-6
-------------------------------------------------
   Change Activity:
                   18-2-6:
-------------------------------------------------
"""

# 37% 写的真垃圾(<_<)  最下是最优写法

# class Solution:
#     def longestCommonPrefix(self, strs):
#         """
#         :type strs: List[str]
#         :rtype: str
#         """
#         size = len(strs)
#         if not size:
#             return ""
#         if size == 1:
#             return strs[0]
#         res = strs[0]
#         for i in strs[1:]:
#             min_len = min(len(i), len(res))
#             for j, item in enumerate(res):
#                 if j > min_len - 1:
#                     res = res[:j]
#                     break
#                 if i[j] != item:
#                     res = res[:j]
#                     break
#         return res


class Solution:
    def longestCommonPrefix(self, strs):
        """
        :type strs: List[str]
        :rtype: str
        """
        if not strs:
            return ''
        for i, chars in enumerate(zip(*strs)):
            if len(set(chars)) > 1:
                return strs[0][:i]
        return min(strs)
