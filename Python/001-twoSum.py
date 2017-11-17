 #1 暴力搜索
"""
class Solution:
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        for m,x in enumerate(nums):
            for n,y in enumerate(nums[m+1:]):
                res = (target-y)
                if x == res:
                    return m,m+1+n
                """

#2 Hash
"""
class Solution:
    def twoSum(self, nums, target):
        buff_dict = {}
        for i in range(len(nums)):
            if nums[i] in buff_dict:
                return [buff_dict[nums[i]], i]
            else:
                buff_dict[target - nums[i]] = i

                """

class Solution:
    def twoSum(self, nums, target):
        dic = {}
        for n,x in enumerate(nums):
            try:
                return dic[x] , n
            except KeyError:
                dic.setdefault(target - x,n)
