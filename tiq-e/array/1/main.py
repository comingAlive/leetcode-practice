class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        l = len(nums)
        s = 0
        for i in range(l):
            # i = 0 ; s = 0 /1
            # i = 1 ; s = 0 > s = 1 > nums s|1 = num i|1    /2
            # i = 2; s = 1 > s = 2 > nums s|2 = num i|2 /3
            # i = 3; s = 2 > s = 2 > nums s|3 = num i|3 /3
            # i = 4 /3
            if nums[i] != nums[s]:
                s += 1
                nums[s] = nums[i]
        return s + 1


s = Solution()
print(s.removeDuplicates([1, 2, 3, 3, 3]))
