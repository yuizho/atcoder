use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let amount: u32 = iter.next().unwrap().parse().unwrap();
    let mut dp = vec![0; 100];

    let mut result = 0;
    for n in 1..100000 {
        let remaining = amount - dp[n - 1];
        if remaining == 0 {
            break;
        }
        let tmp = cmp::max(max_conins(9, remaining), max_conins(6, remaining));
        dp[n] = cmp::max(tmp, 1) + dp[n - 1];
        result += 1;
    }
    println!("{}", result);
}

fn max_conins(coin: u32, amount: u32) -> u32 {
    let mut vec = Vec::new();
    for n in 1..amount {
        let val = coin.pow(n);
        if val <= amount {
            vec.push(val);
        } else {
            break;
        }
    }
    match vec.iter().max() {
        Some(n) => *n,
        None => 0,
    }
}

#[test]
fn test_max_coins() {
    assert_eq!(max_conins(9, 127), 81);
    assert_eq!(max_conins(9, 81), 81);
    assert_eq!(max_conins(9, 9), 9);
    assert_eq!(max_conins(9, 6), 0);
}
