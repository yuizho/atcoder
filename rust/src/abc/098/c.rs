use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let length: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().parse().unwrap();

    let mut easts: Vec<usize> = (0..length).map(|_| 0).collect();
    let mut wests: Vec<usize> = (0..length).map(|_| 0).collect();

    let chars = s.chars().collect::<Vec<char>>();
    for i in (0..length - 1).rev() {
        let c = chars[i + 1];
        if c == 'E' {
            easts[i] = easts[i + 1] + 1;
        } else {
            easts[i] = easts[i + 1];
        }
    }
    for i in 1..length {
        let c = chars[i - 1];
        if c == 'W' {
            wests[i] = wests[i - 1] + 1;
        } else {
            wests[i] = wests[i - 1];
        }
    }

    let results: Vec<usize> = easts.iter().zip(wests.iter()).map(|x| x.0 + x.1).collect();

    println!("{}", results.iter().min().unwrap());
}
