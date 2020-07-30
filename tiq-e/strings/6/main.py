from time import perf_counter


class Solution:
    def myAtoi(self, str: str) -> int:
        my_str = str.lstrip()
        if not my_str:
            return 0
        if my_str[0].isdigit():
            return min(self.helper(my_str, 0, len(my_str)), 2147483647)
        if my_str[0] == '+':
            return min(self.helper(my_str, 1, len(my_str)), 2147483647)
        if my_str[0] == '-':
            return max(-self.helper(my_str, 1, len(my_str)), -2147483648)
        return 0

    @staticmethod
    def helper(my_str, p, length):
        res = 0
        while p < length and my_str[p].isdigit():
            res = res * 10 + int(my_str[p])
            p += 1
        return res


s = "42"

start = perf_counter()
result = Solution().myAtoi(s)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
