use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let items: usize = iter.next().unwrap().parse().unwrap();
    let points: Vec<(i32, i32)> = (0..items)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let mut max_length: f64 = 0.0;
    for point in points.iter() {
        for next in points[1..].iter() {
            let x = (next.0 - point.0).pow(2) as f64;
            let y = (next.1 - point.1).pow(2) as f64;
            let length = (x + y).sqrt();
            if length > max_length {
                max_length = length;
            }
        }
    }
    println!("{:.6}", max_length);
}
