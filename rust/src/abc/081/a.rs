use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let result = buf.chars().filter(|c| *c == '1').count();
    println!("{}", result);
}
