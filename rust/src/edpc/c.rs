use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let days: usize = iter.next().unwrap().parse().unwrap();
    let hapinesses: Vec<(u32, u32, u32)> = (0..days)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut dp = vec![vec![0; 3]; days];
    dp[0] = vec![hapinesses[0].0, hapinesses[0].1, hapinesses[0].2];

    for i in 1..days {
        dp[i][0] = hapinesses[i].0 + cmp::max(dp[i - 1][1], dp[i - 1][2]);
        dp[i][1] = hapinesses[i].1 + cmp::max(dp[i - 1][0], dp[i - 1][2]);
        dp[i][2] = hapinesses[i].2 + cmp::max(dp[i - 1][0], dp[i - 1][1]);
    }
    match dp[days - 1].iter().max() {
        Some(n) => println!("{}", n),
        None => println!("error"),
    }
}
