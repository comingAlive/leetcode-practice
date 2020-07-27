from time import perf_counter


class Solution:
    def firstUniqChar(self, s: str) -> int:
        m = {}
        for i in s:
            m[i] = m.get(i, 0) + 1
            print(i)
        for i in range(len(s)):
            if m[s[i]] == 1:
                return i
        return -1


s = "loveleetcode"

start = perf_counter()
result = Solution().firstUniqChar(s)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
