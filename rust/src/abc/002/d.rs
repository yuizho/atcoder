use std::cmp;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let polititian_number: usize = iter.next().unwrap().parse().unwrap();
    let relation_number: usize = iter.next().unwrap().parse().unwrap();
    let mut N = vec![vec![0; 12]; 12];
    for _ in 0..relation_number {
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        N[x - 1][y - 1] = 1;
        N[y - 1][x - 1] = 1;
    }

    let mut combinations = vec![];
    // 2の3乗の組み合わせを作成
    for i in 0..2usize.pow(polititian_number as u32) {
        let mut combination = vec![];
        // ビット演算する際の最大桁が1桁目に来るとこで
        // シフトできれば良いので、number.len()-1までloop
        for j in 0..polititian_number {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                combination.push(j);
            }
        }
        if combination.len() > 1 {
            combinations.push(combination);
        }
    }

    let mut result = 0;
    for combination in combinations {
        let mut check_results = vec![];
        for p in get_pattern(&combination) {
            let x = combination[p[0]];
            let y = combination[p[1]];
            check_results.push(N[x][y] == 1 && N[y][x] == 1);
        }
        if check_results.iter().all(|&r| r == true) {
            result = cmp::max(combination.len(), result);
        }
    }
    println!("{}", result);
}

fn get_pattern(comb: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut results: Vec<Vec<usize>> = vec![];
    for i in 0..2usize.pow(comb.len() as u32) {
        let mut result = vec![];
        for j in 0..comb.len() {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                result.push(j);
            }
        }
        if result.len() == 2 {
            results.push(result);
        }
    }
    return results;
}
