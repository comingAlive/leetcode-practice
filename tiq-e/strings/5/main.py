from time import perf_counter


class Solution:
    def isPalindrome(self, s):
        s = ''.join(c for c in s if c.isalnum()).lower()
        return s == s[::-1]


s = "ab_a"

start = perf_counter()
result = Solution().isPalindrome(s)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
