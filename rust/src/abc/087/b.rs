use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let count_500: u32 = iter.next().unwrap().parse().unwrap();
    let count_100: u32 = iter.next().unwrap().parse().unwrap();
    let count_50: u32 = iter.next().unwrap().parse().unwrap();
    let amount: u32 = iter.next().unwrap().parse().unwrap();

    let mut result = 0;
    for c500 in 0..count_500 + 1 {
        for c100 in 0..count_100 + 1 {
            for c50 in 0..count_50 + 1 {
                if 500 * c500 + 100 * c100 + 50 * c50 == amount {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
