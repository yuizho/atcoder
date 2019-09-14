use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let K: i32 = iter.next().unwrap().parse().unwrap();
    let S: i32 = iter.next().unwrap().parse().unwrap();

    let mut result = 0;
    for x in 0..K + 1 {
        for y in 0..K + 1 {
            let z = S - x - y;
            if z >= 0 && z <= K {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
