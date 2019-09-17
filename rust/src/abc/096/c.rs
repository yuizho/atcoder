use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let y: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let squares: Vec<Vec<char>> = (0..y)
        .map(|_| {
            let str: String = iter.next().unwrap().parse().unwrap();
            str.chars().collect::<Vec<char>>()
        })
        .collect();

    let mut can_paint = true;
    'outer: for (i, line) in squares.iter().enumerate() {
        for (j, &square) in line.iter().enumerate() {
            if square == '#' {
                // before line
                if i != 0 {
                    if squares[i - 1][j] == '#' {
                        continue;
                    }
                }
                // same line
                if j != 0 {
                    if squares[i][j - 1] == '#' {
                        continue;
                    }
                }
                if j < x - 1 {
                    if squares[i][j + 1] == '#' {
                        continue;
                    }
                }
                // next line
                if i < y - 1 {
                    if squares[i + 1][j] == '#' {
                        continue;
                    }
                }
                //  when the proces reaches here, this squres can't fill.
                can_paint = false;
                break 'outer;
            }
        }
    }
    println!("{}", if can_paint { "Yes" } else { "No" });
}
