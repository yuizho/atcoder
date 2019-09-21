use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let buttons: Vec<usize> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut cnt = 0;
    let mut target_button_index = 1;
    while target_button_index != 2 && cnt < 100000 {
        target_button_index = buttons[target_button_index - 1];
        cnt += 1;
    }
    if cnt == 100000 {
        println!("-1");
    } else {
        println!("{}", cnt);
    }
}
