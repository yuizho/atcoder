use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let polititian_number: usize = iter.next().unwrap().parse().unwrap();
    let relation_number: usize = iter.next().unwrap().parse().unwrap();
    let relations: Vec<(usize, usize)> = (0..relation_number)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut results = vec![];
    // 2の3乗の組み合わせを作成
    for i in 0..2usize.pow(polititian_number as u32) {
        let mut combination = HashSet::new();
        // ビット演算する際の最大桁が1桁目に来るとこで
        // シフトできれば良いので、number.len()-1までloop
        for j in 0..polititian_number {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                combination.insert(j + 1);
            }
        }
        results.push(combination);
    }
    println!("{:?}", results);
}
