use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let length: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().parse().unwrap();

    let mut result = 100000 * 3;
    for n in 0..length {
        let mut count = 0;
        if n != 0 {
            count += s[0..n].chars().filter(|&c| c == 'W').count();
        }
        if n != s.len() - 1 {
            count += s[(n + 1)..].chars().filter(|&c| c == 'E').count();
        }
        if result > count {
            result = count;
        }
    }

    println!("{}", result);
}
