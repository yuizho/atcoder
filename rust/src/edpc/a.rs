use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let steps: usize = iter.next().unwrap().parse().unwrap();
    let costs: Vec<i32> = (0..steps)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let mut dp = vec![0; steps];

    for step in 1..steps {
        if step == 1 {
            dp[step] = i32::abs(costs[step - 1] - costs[step]);
            continue;
        }
        let one_step_cost = dp[step - 1] + i32::abs(costs[step - 1] - costs[step]);
        let tow_step_cost = dp[step - 2] + i32::abs(costs[step - 2] - costs[step]);
        dp[step] = if one_step_cost < tow_step_cost {
            one_step_cost
        } else {
            tow_step_cost
        }
    }
    println!("{:?}", dp[steps - 1]);
}
