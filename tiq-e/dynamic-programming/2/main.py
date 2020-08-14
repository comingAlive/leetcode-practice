from time import perf_counter
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        minprice = float("inf")
        maxprofit = 0
        for price in prices:
            if price < minprice:
                minprice = price
            elif price - minprice > maxprofit:
                maxprofit = price - minprice
        return maxprofit


n = [7, 1, 5, 3, 6, 4]

start = perf_counter()
result = Solution().maxProfit(n)
end = perf_counter()
print("Duration: " + str(end - start))
print("Result: " + str(result))
