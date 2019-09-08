use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let N: u8 = iter.next().unwrap().parse().unwrap();
    let mochis: HashSet<u8> = (0..N)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    println!("{}", mochis.len());
}
