use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    // 途中の計算でmodをとっても大丈夫
    let result = (1..n + 1).fold(1, |acc, num| (acc * num) % 1000000007);
    println!("{}", result);
}
