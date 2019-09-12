use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let number_tuple: Vec<(i32, i32, i32)> = (0..n)
        .map(|_| {
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut prev_numbers = (0, 0, 0);
    let mut result = "Yes";
    for numbers in number_tuple {
        let time_diff = numbers.0 - prev_numbers.0;
        let x_diff = i32::abs(numbers.1 - prev_numbers.1);
        let y_diff = i32::abs(numbers.2 - prev_numbers.2);
        if time_diff < x_diff + y_diff {
            result = "No";
            break;
        }
        if (time_diff % 2 == 0 && (x_diff + y_diff) % 2 != 0)
            || (time_diff % 2 != 0 && (x_diff + y_diff) % 2 == 0)
        {
            result = "No";
            break;
        }
        prev_numbers = numbers;
    }
    println!("{}", result);
}
