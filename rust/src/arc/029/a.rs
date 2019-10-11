use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let meats_number: usize = iter.next().unwrap().parse().unwrap();
    let meats: Vec<usize> = (0..meats_number)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let mut results = vec![];
    // 2の3乗の組み合わせを作成
    for i in 0..2usize.pow(meats_number as u32) {
        let mut grill_a = 0;
        let mut grill_b = 0;
        // ビット演算する際の最大桁が1桁目に来るとこで
        // シフトできれば良いので、number.len()-1までloop
        for j in 0..meats_number {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                grill_a += meats[j];
            } else {
                grill_b += meats[j];
            }
        }
        results.push(cmp::max(grill_a, grill_b));
    }
    println!("{}", results.iter().min().unwrap());
}
