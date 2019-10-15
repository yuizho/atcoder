use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 行ごとのiterが取れる
    let mut iter = buf.split_whitespace();
    let height: usize = iter.next().unwrap().parse().unwrap();
    let width: usize = iter.next().unwrap().parse().unwrap();
    let table: Vec<Vec<char>> = (0..height)
        .map(|_| {
            let row: String = iter.next().unwrap().parse().unwrap();
            row.chars().collect()
        })
        .collect();
    let mut reached = vec![vec![false; width]; height];

    let mut start_x = 0;
    let mut start_y = 0;
    'outer: for (y, row) in table.iter().enumerate() {
        for (x, &column) in row.iter().enumerate() {
            if column == 's' {
                start_x = x;
                start_y = y;
                break 'outer;
            }
        }
    }
    let result = search(
        start_x as i32,
        start_y as i32,
        width,
        height,
        &table,
        &mut reached,
    );
    println!("{}", if result { "yes" } else { "No" });
}

fn search(
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    table: &Vec<Vec<char>>,
    reached: &mut Vec<Vec<bool>>,
) -> bool {
    if x < 0 || width <= x as usize || y < 0 || height <= y as usize {
        return false;
    }
    if table[y as usize][x as usize] == '#' {
        return false;
    }
    if table[y as usize][x as usize] == 'g' {
        return true;
    }
    if reached[y as usize][x as usize] {
        return false;
    }
    reached[y as usize][x as usize] = true;
    if search(x + 1, y, width, height, table, reached)
        || search(x - 1, y, width, height, table, reached)
        || search(x, y + 1, width, height, table, reached)
        || search(x, y - 1, width, height, table, reached)
    {
        return true;
    }
    false
}
