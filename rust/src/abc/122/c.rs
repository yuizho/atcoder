use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let length: usize = iter.next().unwrap().parse().unwrap();
    let inquiries: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().parse().unwrap();
    let ranges: Vec<(usize, usize)> = (0..inquiries)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut cumulative_sum: Vec<usize> = (0..length).map(|_| 0).collect();

    for n in 1..length {
        if s[n - 1..n + 1] == "AC".to_string() {
            cumulative_sum[n] = cumulative_sum[n - 1] + 1;
        } else {
            cumulative_sum[n] = cumulative_sum[n - 1];
        }
    }

    for range in ranges {
        println!(
            "{}",
            cumulative_sum[range.1 - 1] - cumulative_sum[range.0 - 1]
        );
    }
}
