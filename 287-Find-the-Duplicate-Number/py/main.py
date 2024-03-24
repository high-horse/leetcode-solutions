class Solution(object):
    def findDuplicate(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        hasSeenNumber = [0] * len(nums)
        for num in nums:
            if hasSeenNumber[num] == -1:
                return num
            hasSeenNumber[num] = -1
        return -1
