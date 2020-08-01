from time import perf_counter


class Solution:
    def countAndSay(self, n: int) -> str:
        str_ = '1'
        for i in range(2, n + 1):
            temp = ''
            count = 1
            for j in range(1, len(str_)):
                if str_[j - 1] != str_[j]:
                    temp += str(count) + str_[j - 1]
                    count = 1
                else:
                    count += 1
            temp += str(count) + str_[-1]
            str_ = temp
        return str_


n = 4

start = perf_counter()
result = Solution().countAndSay(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
