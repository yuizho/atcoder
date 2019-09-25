use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let squares: Vec<Vec<u32>> = vec![
        (0..n)
            .map(|_| iter.next().unwrap().parse().unwrap())
            .collect(),
        (0..n)
            .map(|_| iter.next().unwrap().parse().unwrap())
            .collect(),
    ];

    let answers: Vec<u32> = (0..n)
        .map(|i| {
            squares[0][0..(i + 1)].iter().fold(0, |acc, num| acc + num)
                + squares[1][i..].iter().fold(0, |acc, num| acc + num)
        })
        .collect();

    println!("{}", answers.iter().max().unwrap());
}
