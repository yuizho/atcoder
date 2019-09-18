use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();

    let start = if a < c { c } else { a };
    let end = if b < d { b } else { d };
    println!("{}", if end - start < 0 { 0 } else { end - start });
}
