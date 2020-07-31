from time import perf_counter


class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if needle == '':
            return 0
        l = len(needle)
        for i in range(len(haystack) - len(needle) + 1):
            if haystack[i:i + l] == needle:
                return i
        return -1


haystack = "hello"
needle = "ll"

start = perf_counter()
result = Solution().strStr(haystack, needle)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
