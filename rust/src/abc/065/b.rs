use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut buttons: Vec<usize> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut result = 1;
    let mut target = 2;
    loop {
        let index_opt = buttons.iter().position(|button_num| *button_num == target);
        if let Some(index) = index_opt {
            let next_button_num = index + 1;
            if next_button_num == 1 {
                break;
            }
            target = next_button_num;
            buttons[index] = 100000;
            result += 1;
        } else {
            result = -1;
            break;
        }
    }
    println!("{}", result);
}
