use std::io::Read;
use std::process::exit;

type Point = (usize, usize);

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let points: usize = iter.next().unwrap().parse().unwrap();
    let lines: usize = iter.next().unwrap().parse().unwrap();
    let mut matrix = vec![vec![0; points]; points];
    let mut starts = vec![0, lines];
    (0..lines).for_each(|x| {
        let i: usize = iter.next().unwrap().parse().unwrap();
        let j: usize = iter.next().unwrap().parse().unwrap();
        matrix[i - 1][j - 1] = 1;
        matrix[j - 1][i - 1] = 1;
        starts[x] = i - 1;
    });
    println!("{:?}", matrix);
    let mut reached = vec![vec![false; points]; points];

    for s in starts {}
}

fn search(matrix: &Vec<Vec<usize>>, reached: &mut Vec<bool>, start_index: usize) -> bool {
    true
}
