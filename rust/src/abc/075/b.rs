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

    let mut result: Vec<Vec<char>> = Vec::new();
    for (i, line) in squares.iter().enumerate() {
        let mut result_line: Vec<char> = Vec::new();
        for (j, &square) in line.iter().enumerate() {
            let mut count = 0;
            if square == '#' {
                result_line.push('#');
                continue;
            }
            count += check_count(&squares[i], j, x);
            if i != 0 {
                count += check_count(&squares[i - 1], j, x);
            }
            if i < y - 1 {
                count += check_count(&squares[i + 1], j, x);
            }
            result_line.push(std::char::from_digit(count, 10).unwrap());
        }
        result.push(result_line);
    }

    result
        .iter()
        .map(|line| line.iter().collect::<String>())
        .for_each(|line| println!("{}", line));
    /*for line in result {
        for word in line {
            print!("{}", word);
        }
        print!("\n");
    }*/
}

fn check_count(line: &Vec<char>, current_index: usize, line_length: usize) -> u32 {
    let mut count = 0;
    if current_index != 0 {
        if line[current_index - 1] == '#' {
            count += 1;
        }
    }
    if line[current_index] == '#' {
        count += 1;
    }
    if current_index < line_length - 1 {
        if line[current_index + 1] == '#' {
            count += 1;
        }
    };
    count
}
