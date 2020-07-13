from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        maxprofit = 0
        if not prices: return 0
        for i in range(len(prices) - 1):
            if prices[i] < prices[i + 1]:
                maxprofit += prices[i + 1] - prices[i]
        return maxprofit


print(Solution().maxProfit([7, 1, 5, 3, 6, 4]))
