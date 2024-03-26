class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        n = len(nums)

        # Swap elements to their corresponding indices
        for i in range(n):
            while 1 <= nums[i] <= n and nums[nums[i] - 1] != nums[i]:
                nums[nums[i] - 1], nums[i] = nums[i], nums[nums[i] - 1]

        # Find the first missing positive integer
        for i in range(n):
            if nums[i] != i + 1:
                return i + 1

        return n + 1
