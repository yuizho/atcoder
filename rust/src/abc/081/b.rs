use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: u8 = iter.next().unwrap().parse().unwrap();
    let mut numbers: Vec<u32> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut attempted_times = 0;
    while numbers.iter().all(|item| item % 2 == 0) {
        numbers = numbers.iter().map(|item| item / 2).collect();
        attempted_times += 1;
    }
    println!("{}", attempted_times);
}
