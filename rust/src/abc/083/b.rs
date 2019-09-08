use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let max: u32 = iter.next().unwrap().parse().unwrap();
    let start: u32 = iter.next().unwrap().parse().unwrap();
    let end: u32 = iter.next().unwrap().parse().unwrap();

    let mut result = 0;
    for num in 1..max + 1 {
        let sum = num
            .to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .sum::<u32>();
        if start <= sum && sum <= end {
            result += num;
        }
    }
    println!("{}", result);
}
