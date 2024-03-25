class Solution:
    def findDuplicates(self, nums: List[int]) -> List[int]:
        duplicates = []
    
        for num in nums:
            # Get the absolute value of the number since we might have made it negative previously
            index = abs(num) - 1
            
            # If the value at index is negative, it means we've already seen this number before
            if nums[index] < 0:
                duplicates.append(index + 1)
            else:
                # Mark the number as visited by making its value negative
                nums[index] = -nums[index]
        
        return duplicates
