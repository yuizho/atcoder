use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let s: String = iter.next().unwrap().parse().unwrap();

    let numbers: Vec<i32> = s.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let mut patterns: Vec<Vec<i32>> = Vec::new();

    for i in 0..2usize.pow(3) {
        let mut row = numbers[1..].to_vec();
        for j in 0..i {
            // j桁右シフトして最初のbitが1かチェック
            if ((i >> j) & 1) == 1 {
                row[j] = -row[j];
            }
        }
        patterns.push(row);
    }

    for p in patterns {
        if p.iter().fold(0, |acc, num| acc + num) + numbers[0] == 7 {
            let formula = p
                .iter()
                .map(|&num| {
                    if num < 0 {
                        num.to_string()
                    } else {
                        format!("{}{}", "+", num.to_string())
                    }
                })
                .collect::<Vec<String>>()
                .join("");
            println!("{}{}=7", numbers[0], formula);
            return;
        }
    }
}
