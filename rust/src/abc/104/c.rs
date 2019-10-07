use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let problem_counts: usize = iter.next().unwrap().parse().unwrap();
    let target_score: usize = iter.next().unwrap().parse().unwrap();
    let mut bornases: Vec<(usize, usize, usize)> = (0..problem_counts)
        .enumerate()
        .map(|(i, _)| {
            (
                (i + 1) * 100,
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .map(|x| (x.0, x.1, x.0 * x.1 + x.2))
        .collect();

    println!("{:?}", bornases);

    // 各行を選ぶ選ばないパターンを作る
    let mut patterns = Vec::new();
    // 2の3乗の組み合わせを作成
    for i in 0..2usize.pow(problem_counts as u32) {
        let mut row = (
            0,
            0,
            (
                bornases[problem_counts - 1].0,
                bornases[problem_counts - 1].1,
            ),
        );
        // ビット演算する際の最大桁が1桁目に来るとこで
        // シフトできれば良いので、number.len()-1までloop
        for j in 0..problem_counts {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                let another_score = if j == problem_counts - 1 {
                    (
                        bornases[problem_counts - 2].0,
                        bornases[problem_counts - 2].1,
                    )
                } else {
                    (
                        bornases[problem_counts - 1].0,
                        bornases[problem_counts - 1].1,
                    )
                };
                row = (row.0 + bornases[j].1, row.1 + bornases[j].2, another_score);
            }
        }
        patterns.push(row);
    }
    patterns = patterns[0..patterns.len() - 1].to_vec();

    println!("{:?}", patterns);

    let mut result = 10000000;
    for pattern in &patterns {
        let mut tmp_count = 0;
        if pattern.1 >= target_score {
            tmp_count = pattern.0;
        } else {
            let mut current_score = pattern.1;
            tmp_count = pattern.0;
            for sub in 0..(pattern.2).1 {
                if current_score < target_score {
                    tmp_count += 1;
                    current_score += (pattern.2).0;
                }
            }
            if current_score < target_score {
                continue;
            }
        }
        if result > tmp_count && tmp_count != 0 {
            result = tmp_count;
        }
    }

    println!("{}", result);
}
