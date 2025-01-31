fn is_match(s: &str, p: &str) -> bool {
    let (m, n) = (s.len(), p.len());
    let s = s.as_bytes();
    let p = p.as_bytes();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
 
    for j in 1..=n {
        if p[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 1];
        }
    }
 
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == b'*' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p[j - 1] == b'?' || s[i - 1] == p[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
 
    dp[m][n]
}
 
pub fn execute() -> () {
    println!("{}", is_match("aa", "a"));          // Output: false
    println!("{}", is_match("aa", "*"));          // Output: true
    println!("{}", is_match("cb", "?a"));         // Output: false
    println!("{}", is_match("", "******"));        // Output: true
    println!("{}", is_match("aa", "a*"));          // Output: true
    println!("{}", is_match("abcabczzzde", "*abc???de*")); // Output: true
    println!("{}", is_match("adceb", "*a*b"));      // Output: true
    println!("{}", is_match("abefcdgiescdfimde", "ab*cd?i*de")); // Output: true
    println!("{}", is_match("a", "aa"));          // Output: false
    println!("{}", is_match("acdcb", "a*c?b"));    // Output: false
}