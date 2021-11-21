//You are given an array prices where prices[i] is the price of a given stock
//on the iᵗʰ day.
//
// You want to maximize your profit by choosing a single day to buy one stock
//and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you
//cannot achieve any profit, return 0.
//
//
// Example 1:
//
//
//Input: prices = [7,1,5,3,6,4]
//Output: 5
//Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit =
//6-1 = 5.
//Note that buying on day 2 and selling on day 1 is not allowed because you
//must buy before you sell.
//
//
// Example 2:
//
//
//Input: prices = [7,6,4,3,1]
//Output: 0
//Explanation: In this case, no transactions are done and the max profit = 0.
//
//
//
// Constraints:
//
//
// 1 <= prices.length <= 10⁵
// 0 <= prices[i] <= 10⁴
//
// Related Topics Array Dynamic Programming 👍 12053 👎 443


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(max_profit(vec![2, 1, 2, 1, 0 ,1 ,2]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(max_profit(vec![9, 10, 2, 5, 1, 3]), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(max_profit(vec![1, 2]), 1);
    }
}


//leetcode submit region begin(Prohibit modification and deletion)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() > 1 {
        let mut prices = prices;
        let mut buy = prices.remove(0);
        let mut profit = 0;

        for p in prices {
            if p < buy {
                buy = p;
            } else {
                profit = profit.max(p - buy);
            }
        }

        profit
    } else {
        0
    }
}
//leetcode submit region end(Prohibit modification and deletion)
