struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut buy = 0;
        let mut sell = 1;

        let mut maxi_profit = 0;

        for _ in 1..prices.len() {
            let profit = prices[sell] - prices[buy];
            if profit > 0 {
                maxi_profit = std::cmp::max(maxi_profit, profit);
            } else {
                buy = sell;
            }
            sell += 1;
        }

        maxi_profit
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
