pub fn longest_common_subsequence(s: &[char], t: &[char]) -> usize {
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    dp[n][m]
}
