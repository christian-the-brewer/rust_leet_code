// problem 121 best time to buy and sell stock
fn main() {

pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_price = &prices[0];
        let mut profit = 0;
        for i in 0..prices.len() {
            if &prices[i] - lowest_price > profit {
            profit = &prices[i] - lowest_price;
            }
            if &prices[i] < lowest_price {
                lowest_price = &prices[i];
            }
        }
        profit
    }
}
