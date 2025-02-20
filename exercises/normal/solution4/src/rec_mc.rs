pub fn dp_rec_mc(amount: u32) -> u32 {
    let coins: Vec<u32> = vec![1, 2, 5, 10, 20, 50, 100];
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;

    for coin in &coins {
        for i in *coin..=(amount as u32) {
            dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - *coin) as usize] + 1);
        }
    }

    dp[amount as usize]
}