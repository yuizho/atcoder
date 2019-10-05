use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let input: String = iter.next().unwrap().parse().unwrap();

    let mut patterns = Vec::new();
    for i in 0..(1 << input.len() - 1) {
        let mut row = Vec::new();
        for j in 0..input.len() {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                row.push(j);
            }
        }
        patterns.push(row);
    }

    let mut result = 0;
    for pattern in patterns {
        let mut left = 0;
        for right in pattern {
            let n: u64 = input[left..right + 1].parse().unwrap();
            result += n;
            left = right + 1;
        }
        let n: u64 = input[left..].parse().unwrap();
        result += n;
    }
    println!("{:?}", result);
}
