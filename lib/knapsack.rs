pub fn knapsack(w: usize, weight: &[usize], value: &[usize]) -> usize {
    let n = weight.len();
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        for j in 0..=w {
            if weight[i] <= j {
                dp[i + 1][j] = dp[i][j].max(dp[i][j - weight[i]] + value[i]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    dp[n][w]
}
