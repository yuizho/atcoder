use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let amount: u32 = iter.next().unwrap().parse().unwrap();
    // Nの最大値 + 1こ作っとく
    let mut dp: Vec<u32> = vec![amount; 100001 as usize];
    dp[0] = 0;

    for n in 1..(amount + 1) {
        let n = n as usize;
        dp[n] = cmp::min(dp[n], dp[n - 1] + 1);

        // その要素番号(金額)までの最小手を求めて現在のdp[n]より小さければそちらを採用する
        fill_min_step_to_dp(&mut dp, 6, n);
        fill_min_step_to_dp(&mut dp, 9, n);
    }
    println!("{}", dp[amount as usize]);
}

fn fill_min_step_to_dp(dp: &mut Vec<u32>, coin: u32, n: usize) {
    for j in 1..n {
        if n as u32 >= coin.pow(j as u32) {
            dp[n] = cmp::min(dp[n], dp[n - coin.pow(j as u32) as usize] + 1);
        } else {
            break;
        }
    }
}
