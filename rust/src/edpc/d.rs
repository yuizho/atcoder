use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let item_count: usize = iter.next().unwrap().parse().unwrap();
    let max_weight: usize = iter.next().unwrap().parse().unwrap();
    let napsack: Vec<(u64, u64)> = (0..item_count)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut dp = vec![vec![0; max_weight + 1]; item_count + 1];

    for i in 0..item_count {
        for w in 0..max_weight + 1 {
            let weight = napsack[i].0 as usize;
            let value = napsack[i].1;
            if w >= weight {
                let current_value = dp[i][w - weight] + value;
                ch_max(&mut dp[i + 1][w], current_value);
            }

            let current_value = dp[i][w];
            ch_max(&mut dp[i + 1][w], current_value);
        }
    }
    println!("{}", dp[item_count][max_weight]);
}

fn ch_max(a: &mut u64, b: u64) -> bool {
    if *a < b {
        *a = b;
        return true;
    }
    return false;
}
