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

    let mut i = 1;
    let mut result = "NO";
    // https://qiita.com/shihou22/items/573251a43b7deb5e6964#b%E5%95%8F%E9%A1%8C
    while i < b {
        if a * i % b == c {
            result = "YES";
            break;
        }
        i += 1;
    }

    println!("{}", result);
}
