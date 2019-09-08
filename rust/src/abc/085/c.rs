use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let number: u32 = iter.next().unwrap().parse().unwrap();
    let amount: u32 = iter.next().unwrap().parse().unwrap();

    let mut result = String::from("-1 -1 -1");
    'loop10000: for c10000 in 0..number + 1 {
        'loop5000: for c5000 in 0..(number - c10000) + 1 {
            let c1000 = number - (c5000 + c10000);
            if c10000 * 10000 + c5000 * 5000 + c1000 * 1000 == amount {
                result = format!(
                    "{} {} {}",
                    c10000.to_string(),
                    c5000.to_string(),
                    c1000.to_string()
                );
                break 'loop10000;
            }
        }
    }
    println!("{}", result);
}
