class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        l = len(nums)
        s = 0
        for i in range(l):
            if nums[i] != nums[s]:
                s += 1
                nums[s] = nums[i]
        return s + 1


print(Solution().removeDuplicates([1, 2, 3, 3, 3]))
