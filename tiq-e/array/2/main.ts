function maxProfit(prices: number[]): number {
  let maxprofit = 0
  if (prices.length > 0) {
    prices.reduce((acc, next) => {
      if (next > acc) {
        maxprofit += next - acc
      }
      return next
    })
  }
  return maxprofit
}

console.log(maxProfit([7, 1, 5, 3, 6, 4]))